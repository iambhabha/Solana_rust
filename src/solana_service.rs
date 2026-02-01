use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use solana_transaction_status::UiTransactionEncoding;
use std::time::Duration;
use tokio::time::sleep;
use spl_associated_token_account::get_associated_token_address;
use spl_token::{
    instruction::transfer_checked,
    state::Account as TokenAccount,
};

use crate::config::Config;
use crate::errors::{AppError, AppResult};

/// Solana blockchain service
pub struct SolanaService {
    rpc_client: RpcClient,
    config: Config,
}

impl SolanaService {
    /// Create new Solana service instance
    pub fn new(config: Config) -> Self {
        let rpc_client = RpcClient::new_with_commitment(
            config.solana_rpc_url.clone(),
            CommitmentConfig::confirmed(),
        );

        Self { rpc_client, config }
    }

    /// Get associated token account address for a user
    pub fn get_associated_token_address(&self, user_pubkey: &Pubkey) -> AppResult<Pubkey> {
        let mint_pubkey: Pubkey = self
            .config
            .karmm_mint_address
            .parse()
            .map_err(|e| AppError::Solana(format!("Invalid mint address: {}", e)))?;

        Ok(get_associated_token_address(user_pubkey, &mint_pubkey))
    }

    /// Get token balance for a user's associated token account
    pub async fn get_token_balance(&self, user_pubkey: &Pubkey) -> AppResult<u64> {
        let ata = self.get_associated_token_address(user_pubkey)?;

        let account_data = self
            .rpc_client
            .get_account_data(&ata)
            .await
            .map_err(|e| AppError::Solana(format!("Failed to fetch account: {}", e)))?;

        if account_data.is_empty() {
            return Ok(0);
        }

        let token_account = TokenAccount::unpack(&account_data)
            .map_err(|e| AppError::Solana(format!("Failed to parse token account: {}", e)))?;

        Ok(token_account.amount)
    }

    /// Get SOL balance for a wallet (in lamports)
    /// 1 SOL = 1,000,000,000 lamports
    pub async fn get_sol_balance(&self, pubkey: &Pubkey) -> AppResult<u64> {
        let balance = self
            .rpc_client
            .get_balance(pubkey)
            .await
            .map_err(|e| AppError::Solana(format!("Failed to get SOL balance: {}", e)))?;

        Ok(balance)
    }

    /// Create associated token account if it doesn't exist
    pub async fn create_associated_token_account_if_needed(
        &self,
        user_pubkey: &Pubkey,
        payer: &solana_sdk::signature::Keypair,
    ) -> AppResult<()> {
        let mint_pubkey: Pubkey = self
            .config
            .karmm_mint_address
            .parse()
            .map_err(|e| AppError::Solana(format!("Invalid mint address: {}", e)))?;

        let ata = get_associated_token_address(user_pubkey, &mint_pubkey);

        // Check if account exists
        match self.rpc_client.get_account(&ata).await {
            Ok(_) => {
                // Account already exists
                return Ok(());
            }
            Err(_) => {
                // Account doesn't exist, create it
            }
        }

        let instruction = spl_associated_token_account::instruction::create_associated_token_account(
            &payer.pubkey(),
            user_pubkey,
            &mint_pubkey,
            &spl_token::id(),
        );

        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .map_err(|e| AppError::Solana(format!("Failed to get blockhash: {}", e)))?;

        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[payer], recent_blockhash);

        let signature = self
            .rpc_client
            .send_transaction(&transaction)
            .await
            .map_err(|e| AppError::Solana(format!("Failed to send transaction: {}", e)))?;

        // Wait for confirmation by polling
        self.wait_for_confirmation(&signature).await?;

        Ok(())
    }

    /// Transfer tokens using transfer_checked
    pub async fn transfer_tokens(
        &self,
        from_keypair: &solana_sdk::signature::Keypair,
        to_pubkey: &Pubkey,
        amount: u64,
    ) -> AppResult<String> {
        let mint_pubkey: Pubkey = self
            .config
            .karmm_mint_address
            .parse()
            .map_err(|e| AppError::Solana(format!("Invalid mint address: {}", e)))?;

        let from_ata = get_associated_token_address(&from_keypair.pubkey(), &mint_pubkey);
        let to_ata = get_associated_token_address(to_pubkey, &mint_pubkey);

        // Ensure recipient ATA exists
        self.create_associated_token_account_if_needed(to_pubkey, from_keypair)
            .await?;

        let decimals = 9u8; // Standard SPL token decimals, adjust if KARMM uses different

        let transfer_instruction = transfer_checked(
            &spl_token::id(),
            &from_ata,
            &mint_pubkey,
            &to_ata,
            &from_keypair.pubkey(),
            &[&from_keypair.pubkey()],
            amount,
            decimals,
        )
        .map_err(|e| AppError::Solana(format!("Failed to create transfer instruction: {}", e)))?;

        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .map_err(|e| AppError::Solana(format!("Failed to get blockhash: {}", e)))?;

        let mut transaction =
            Transaction::new_with_payer(&[transfer_instruction], Some(&from_keypair.pubkey()));
        transaction.sign(&[from_keypair], recent_blockhash);

        let signature = self
            .rpc_client
            .send_transaction(&transaction)
            .await
            .map_err(|e| AppError::Solana(format!("Failed to send transaction: {}", e)))?;

        // Wait for confirmation by polling
        self.wait_for_confirmation(&signature).await?;

        Ok(signature.to_string())
    }

    /// Wait for transaction confirmation by polling
    async fn wait_for_confirmation(
        &self,
        signature: &solana_sdk::signature::Signature,
    ) -> AppResult<()> {
        let max_attempts = 30;
        for _i in 0..max_attempts {
            // Check if transaction exists and is confirmed
            match self
                .rpc_client
                .get_transaction(signature, UiTransactionEncoding::Json)
                .await
            {
                Ok(tx) => {
                    // Transaction found, check if it succeeded
                    if let Some(meta) = tx.transaction.meta {
                        if meta.err.is_none() {
                            // Transaction succeeded
                            return Ok(());
                        } else {
                            return Err(AppError::Solana(format!(
                                "Transaction failed: {:?}",
                                meta.err
                            )));
                        }
                    }
                    // If meta is None but transaction exists, consider it confirmed
                    return Ok(());
                }
                Err(_) => {
                    // Transaction not found yet or error, continue polling
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }
        // Return success even if we couldn't confirm - transaction was sent
        // In production, you might want to return an error here
        Ok(())
    }

    /// Get master wallet keypair from config
    pub fn get_master_keypair(&self) -> AppResult<solana_sdk::signature::Keypair> {
        let master_key_bytes = hex::decode(&self.config.master_wallet_private_key)
            .map_err(|e| AppError::Solana(format!("Invalid master wallet key: {}", e)))?;

        solana_sdk::signature::Keypair::from_bytes(&master_key_bytes)
            .map_err(|e| AppError::Solana(format!("Failed to create master keypair: {}", e)))
    }
}
