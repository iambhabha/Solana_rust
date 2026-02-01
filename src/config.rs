use std::env;

/// Application configuration loaded from environment variables
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub encryption_key: String,
    pub solana_rpc_url: String,
    pub karmm_mint_address: String,
    pub master_wallet_private_key: String,
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    pub fn load() -> Self {
        dotenv::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "users.db".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set in .env file"),
            encryption_key: env::var("ENCRYPTION_KEY")
                .expect("ENCRYPTION_KEY must be set in .env file (32 bytes hex encoded)"),
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .expect("SOLANA_RPC_URL must be set in .env file"),
            karmm_mint_address: env::var("KARMM_MINT_ADDRESS")
                .expect("KARMM_MINT_ADDRESS must be set in .env file"),
            master_wallet_private_key: env::var("MASTER_WALLET_PRIVATE_KEY")
                .expect("MASTER_WALLET_PRIVATE_KEY must be set in .env file"),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid number"),
        }
    }

    /// Get encryption key as bytes
    pub fn encryption_key_bytes(&self) -> Vec<u8> {
        hex::decode(&self.encryption_key)
            .expect("ENCRYPTION_KEY must be valid hex (64 characters for 32 bytes)")
    }
}
