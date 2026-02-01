use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use hex;

use crate::errors::{AppError, AppResult};

/// Encrypt private key before storing in database
pub fn encrypt_private_key(key_bytes: &[u8], encryption_key: &[u8]) -> AppResult<String> {
    if encryption_key.len() != 32 {
        return Err(AppError::Encryption(
            "Encryption key must be 32 bytes".to_string(),
        ));
    }

    let key = Key::<Aes256Gcm>::from_slice(encryption_key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, key_bytes)
        .map_err(|e| AppError::Encryption(format!("Encryption failed: {}", e)))?;

    // Combine nonce and ciphertext: nonce (12 bytes) + ciphertext
    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(hex::encode(combined))
}

/// Decrypt private key from database
pub fn decrypt_private_key(encrypted_hex: &str, encryption_key: &[u8]) -> AppResult<Vec<u8>> {
    if encryption_key.len() != 32 {
        return Err(AppError::Encryption(
            "Encryption key must be 32 bytes".to_string(),
        ));
    }

    let combined = hex::decode(encrypted_hex)
        .map_err(|e| AppError::Encryption(format!("Invalid hex: {}", e)))?;

    if combined.len() < 12 {
        return Err(AppError::Encryption("Invalid encrypted data".to_string()));
    }

    let nonce = Nonce::from_slice(&combined[..12]);
    let ciphertext = &combined[12..];

    let key = Key::<Aes256Gcm>::from_slice(encryption_key);
    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| AppError::Encryption(format!("Decryption failed: {}", e)))?;

    Ok(plaintext)
}
