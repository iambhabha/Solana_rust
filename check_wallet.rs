// Check master wallet balance and verify configuration
// Usage: cargo run --bin check_wallet

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, 
    pubkey::Pubkey, 
    signature::{Keypair, Signer}
};
use std::str::FromStr;

fn main() {
    println!("🔍 Checking Master Wallet Configuration...\n");

    // Load environment variables
    dotenv::dotenv().ok();

    // Get configuration
    let rpc_url = std::env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL not found in .env");
    let mint_address = std::env::var("KARMM_MINT_ADDRESS").expect("KARMM_MINT_ADDRESS not found in .env");
    let master_key_hex = std::env::var("MASTER_WALLET_PRIVATE_KEY")
        .expect("MASTER_WALLET_PRIVATE_KEY not found in .env");

    println!("📡 RPC URL: {}", rpc_url);
    println!("🪙  Mint Address: {}", mint_address);
    println!();

    // Decode master wallet private key
    let private_key_bytes = match hex::decode(&master_key_hex) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("❌ Error: Failed to decode private key from hex: {}", e);
            println!("   Make sure MASTER_WALLET_PRIVATE_KEY is in hex format (128 characters)");
            return;
        }
    };

    if private_key_bytes.len() != 64 {
        println!("❌ Error: Private key must be 64 bytes (128 hex chars), got {} bytes", private_key_bytes.len());
        return;
    }

    // Create keypair
    let master_keypair = match Keypair::from_bytes(&private_key_bytes) {
        Ok(kp) => kp,
        Err(e) => {
            println!("❌ Error: Failed to create keypair: {}", e);
            return;
        }
    };

    let master_pubkey = master_keypair.pubkey();
    println!("✅ Master Wallet Public Key: {}", master_pubkey);
    println!("   Explorer: https://explorer.solana.com/address/{}", master_pubkey);
    println!();

    // Connect to RPC
    println!("🔌 Connecting to Solana RPC...");
    let rpc_client = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());

    // Check SOL balance
    println!("💰 Checking SOL balance...");
    match rpc_client.get_balance(&master_pubkey) {
        Ok(balance) => {
            let sol_balance = balance as f64 / 1_000_000_000.0;
            println!("   SOL Balance: {} SOL", sol_balance);
            
            if sol_balance < 0.01 {
                println!("   ⚠️  WARNING: Balance too low! Need at least 0.01 SOL for transactions.");
                println!("   💡 Add SOL to this address: {}", master_pubkey);
            } else if sol_balance < 0.1 {
                println!("   ⚠️  Balance is low. Recommended: 0.1+ SOL");
            } else {
                println!("   ✅ Balance is sufficient");
            }
        }
        Err(e) => {
            println!("   ❌ Error fetching balance: {}", e);
            println!("   Check if RPC URL is correct and accessible");
            return;
        }
    }
    println!();

    // Verify mint address
    println!("🪙  Verifying KARMM token mint...");
    match Pubkey::from_str(&mint_address) {
        Ok(mint_pubkey) => {
            println!("   ✅ Mint address format is valid");
            println!("   Explorer: https://explorer.solana.com/address/{}", mint_address);
            
            // Try to fetch mint account
            match rpc_client.get_account(&mint_pubkey) {
                Ok(account) => {
                    println!("   ✅ Mint account exists on blockchain");
                    println!("   Account owner: {}", account.owner);
                }
                Err(e) => {
                    println!("   ❌ Error: Mint account not found or invalid: {}", e);
                    println!("   Make sure this is the correct KARMM token mint address");
                }
            }
        }
        Err(e) => {
            println!("   ❌ Error: Invalid mint address format: {}", e);
            println!("   Make sure KARMM_MINT_ADDRESS is a valid base58 Solana address");
        }
    }
    println!();

    // Summary
    println!("📋 Summary:");
    println!("─────────────────────────────────────────");
    println!("1. RPC Connection: Check logs above");
    println!("2. Master Wallet: {}", master_pubkey);
    println!("3. SOL Balance: Check logs above");
    println!("4. Token Mint: {}", mint_address);
    println!();
    println!("💡 Next Steps:");
    println!("   • If SOL balance is low, add SOL to: {}", master_pubkey);
    println!("   • If mint is invalid, update KARMM_MINT_ADDRESS in .env");
    println!("   • Once fixed, restart server: cargo run");
}
