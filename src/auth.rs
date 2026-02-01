use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::{AppError, AppResult};

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i64,
    pub email: String,
    pub exp: usize,
}

impl Claims {
    /// Create new claims with expiration
    pub fn new(user_id: i64, email: String) -> Self {
        let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
        Self { user_id, email, exp }
    }
}

/// Generate JWT token for user
pub fn generate_token(user_id: i64, email: &str, secret: &str) -> AppResult<String> {
    let claims = Claims::new(user_id, email.to_string());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::Auth(format!("Token generation failed: {}", e)))?;

    Ok(token)
}

/// Verify and decode JWT token
pub fn verify_token(token: &str, secret: &str) -> AppResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Auth(format!("Token verification failed: {}", e)))?;

    Ok(token_data.claims)
}

use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use hex;

const SALT_LENGTH: usize = 32;
const ITERATIONS: u32 = 100000;

/// Hash password using PBKDF2
pub fn hash_password(password: &str) -> AppResult<String> {
    use rand::RngCore;
    
    // Generate random salt
    let mut salt = [0u8; SALT_LENGTH];
    rand::thread_rng().fill_bytes(&mut salt);
    
    // Hash password with PBKDF2
    let mut hash = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, ITERATIONS, &mut hash);
    
    // Combine salt and hash: salt (hex) + ":" + hash (hex)
    let salt_hex = hex::encode(salt);
    let hash_hex = hex::encode(hash);
    Ok(format!("{}:{}", salt_hex, hash_hex))
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    // Parse salt and hash from stored format
    let parts: Vec<&str> = hash.split(':').collect();
    if parts.len() != 2 {
        return Err(AppError::Auth("Invalid hash format".to_string()));
    }
    
    let salt_hex = parts[0];
    let stored_hash_hex = parts[1];
    
    // Decode salt
    let salt = hex::decode(salt_hex)
        .map_err(|e| AppError::Auth(format!("Invalid salt format: {}", e)))?;
    
    // Hash password with same salt
    let mut computed_hash = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, ITERATIONS, &mut computed_hash);
    let computed_hash_hex = hex::encode(computed_hash);
    
    // Constant-time comparison
    Ok(computed_hash_hex == stored_hash_hex)
}
