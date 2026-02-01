use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
};

use crate::encryption::{decrypt_private_key, encrypt_private_key};
use crate::errors::{AppError, AppResult};

/// Create a new Solana wallet keypair
pub fn create_wallet() -> (String, Vec<u8>) {
    let keypair = Keypair::new();
    let public_key = keypair.pubkey().to_string();
    let private_key_bytes = keypair.to_bytes().to_vec();
    (public_key, private_key_bytes)
}

/// Create keypair from encrypted private key
pub fn keypair_from_encrypted(
    encrypted_private_key: &str,
    encryption_key: &[u8],
) -> AppResult<Keypair> {
    let private_key_bytes = decrypt_private_key(encrypted_private_key, encryption_key)?;

    if private_key_bytes.len() != 64 {
        return Err(AppError::Encryption(
            "Invalid private key length".to_string(),
        ));
    }

    let keypair = Keypair::from_bytes(&private_key_bytes)
        .map_err(|e| AppError::Encryption(format!("Failed to create keypair: {}", e)))?;

    Ok(keypair)
}

/// Encrypt and format private key for storage
pub fn encrypt_keypair_for_storage(
    keypair: &Keypair,
    encryption_key: &[u8],
) -> AppResult<String> {
    let private_key_bytes = keypair.to_bytes();
    encrypt_private_key(&private_key_bytes, encryption_key)
}

/// Parse public key from string
pub fn parse_pubkey(pubkey_str: &str) -> AppResult<Pubkey> {
    pubkey_str
        .parse()
        .map_err(|e| AppError::Validation(format!("Invalid public key: {}", e)))
}
