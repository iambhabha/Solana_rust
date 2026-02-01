use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::auth::{generate_token, hash_password, verify_password, Claims};
use crate::config::Config;
use crate::db::Database;
use crate::errors::AppError;
use crate::solana_service::SolanaService;
use crate::wallet::{create_wallet, encrypt_keypair_for_storage, keypair_from_encrypted, parse_pubkey};
use hex;
use solana_sdk::signature::Signer;

/// Application state shared across routes
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub solana_service: Arc<SolanaService>,
    pub config: Arc<Config>,
}

/// Request/Response types
#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
    pub user_id: i64,
    pub public_key: String,
    pub token: String,
    pub solana_explorer_url: String,
    pub phantom_wallet_info: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
    pub user_id: i64,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub balance: u64,
    pub balance_formatted: String,
    pub public_key: String,
    pub solana_explorer_url: String,
    pub phantom_wallet_import_info: String,
}

#[derive(Deserialize)]
pub struct BuyTokenRequest {
    #[serde(default)]
    pub amount: Option<u64>, // Optional - use tokens if provided
    #[serde(default)]
    pub tokens: Option<f64>, // User-friendly: 5.0 tokens = 5 tokens
    #[serde(default)]
    pub payment_id: Option<String>,
}

#[derive(Serialize)]
pub struct BuyTokenResponse {
    pub message: String,
    pub transaction_signature: String,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct TransferRequest {
    pub to: String,
    #[serde(default)]
    pub amount: Option<u64>, // Optional - use tokens if provided
    #[serde(default)]
    pub tokens: Option<f64>, // User-friendly: 5.0 tokens = 5 tokens
}

#[derive(Serialize)]
pub struct TransferResponse {
    pub message: String,
    pub transaction_signature: String,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct DeductRequest {
    pub user_id: Option<i64>,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct DeductResponse {
    pub message: String,
    pub transaction_signature: String,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct RewardRequest {
    pub user_id: i64,
    #[serde(default)]
    pub amount: Option<u64>, // Optional - use tokens if provided
    #[serde(default)]
    pub tokens: Option<f64>, // User-friendly: 5.0 tokens = 5 tokens
}

#[derive(Serialize)]
pub struct RewardResponse {
    pub message: String,
    pub transaction_signature: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub status: u16,
}

#[derive(Deserialize)]
pub struct TestTransferToMasterRequest {
    pub from_private_key: String, // Hex format (128 characters)
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct TransferToMasterByAddressRequest {
    pub from_address: String, // Solana public key (Base58)
    pub amount: u64,
}

#[derive(Serialize)]
pub struct TestTransferToMasterResponse {
    pub message: String,
    pub transaction_signature: String,
    pub amount: u64,
    pub from_address: String,
    pub to_address: String,
}

/// Send to address request (master to any Solana address)
#[derive(Deserialize)]
pub struct SendToAddressRequest {
    pub to_address: String,  // Solana public key (base58)
    #[serde(default)]
    pub amount: Option<u64>, // Optional - use tokens if provided
    #[serde(default)]
    pub tokens: Option<f64>, // User-friendly: 5.0 tokens = 5 tokens
}

/// Send to address response
#[derive(Serialize)]
pub struct SendToAddressResponse {
    pub message: String,
    pub transaction_signature: String,
    pub amount: u64,
    pub to_address: String,
    pub solana_explorer_url: String,
}

/// Master balance response (admin only)
#[derive(Serialize)]
pub struct MasterBalanceResponse {
    pub message: String,
    pub master_public_key: String,
    pub sol_balance: f64,
    pub token_balance_raw: u64,
    pub token_balance_formatted: String,
    pub token_mint: String,
    pub solana_explorer_url: String,
    pub status: String,
}

/// POST /signup - Create new user and wallet
pub async fn signup(
    State(state): State<AppState>,
    Json(req): Json<SignupRequest>,
) -> Result<Json<SignupResponse>, AppError> {
    // Validate email format (basic check)
    if !req.email.contains('@') {
        return Err(AppError::Validation("Invalid email format".to_string()));
    }

    // Check if email already exists
    if state.db.email_exists(&req.email)? {
        return Err(AppError::Validation("Email already registered".to_string()));
    }

    // Validate password length
    if req.password.len() < 8 {
        return Err(AppError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Create Solana wallet
    let (public_key, private_key_bytes) = create_wallet();

    // Hash password
    let password_hash = hash_password(&req.password)?;

    // Encrypt private key
    let encrypted_private_key = encrypt_keypair_for_storage(
        &solana_sdk::signature::Keypair::from_bytes(&private_key_bytes)
            .map_err(|e| AppError::Internal(format!("Failed to create keypair: {}", e)))?,
        &state.config.encryption_key_bytes(),
    )?;

    // Save user to database
    let user_id = state.db.save_user(
        &req.email,
        &password_hash,
        &public_key,
        &encrypted_private_key,
    )?;

    // Generate JWT token
    let token = generate_token(user_id, &req.email, &state.config.jwt_secret)?;

    // Create associated token account for the user
    let user_pubkey = parse_pubkey(&public_key)?;
    let master_keypair = state.solana_service.get_master_keypair()?;
    
    // This will create ATA if it doesn't exist
    state
        .solana_service
        .create_associated_token_account_if_needed(&user_pubkey, &master_keypair)
        .await
        .map_err(|e| AppError::Solana(format!("Failed to create token account: {}", e)))?;

    // Solana explorer URL (Official Solana Explorer)
    let explorer_url = format!("https://explorer.solana.com/address/{}", public_key);
    
    // Phantom wallet info
    let phantom_info = format!(
        "✅ Real Solana Mainnet Wallet Created! \
        \n📱 To view in Phantom Wallet: \
        \n   1. Open Phantom app \
        \n   2. Go to Settings > Add/Connect Wallet \
        \n   3. Import using private key (contact support for private key export) \
        \n   4. Or send tokens to this address: {} \
        \n\n🔗 View on Solana Explorer: {} \
        \n\n⚠️ This is a REAL mainnet wallet - all transactions are REAL!",
        public_key, explorer_url
    );

    Ok(Json(SignupResponse {
        message: "User created successfully".to_string(),
        user_id,
        public_key,
        token,
        solana_explorer_url: explorer_url,
        phantom_wallet_info: phantom_info,
    }))
}

/// POST /login - Authenticate user
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = state.db.get_user_by_email(&req.email)?;

    // Verify password
    let is_valid = verify_password(&req.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::Auth("Invalid email or password".to_string()));
    }

    // Generate JWT token
    let token = generate_token(user.id, &user.email, &state.config.jwt_secret)?;

    Ok(Json(LoginResponse {
        message: "Login successful".to_string(),
        token,
        user_id: user.id,
    }))
}

/// Custom extractor for Claims from JWT token
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or_else(|| AppError::Auth("Unauthorized - Missing or invalid token".to_string()))
    }
}

/// GET /balance - Get user's token balance (requires auth)
pub async fn get_balance(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<BalanceResponse>, AppError> {
    let user = state.db.get_user_by_id(claims.user_id)?;
    let user_pubkey = parse_pubkey(&user.public_key)?;

    let balance = state.solana_service.get_token_balance(&user_pubkey).await?;

    // Format balance with 9 decimals (standard SPL token)
    let decimals = 9u32;
    let balance_formatted = format!("{:.9}", balance as f64 / 10_f64.powi(decimals as i32));
    
    // Solana explorer URL (Official Solana Explorer)
    let explorer_url = format!("https://explorer.solana.com/address/{}", user.public_key);
    
    // Phantom wallet import instructions
    let phantom_info = format!(
        "To view in Phantom: Import account with public key: {}",
        user.public_key
    );

    Ok(Json(BalanceResponse {
        balance,
        balance_formatted,
        public_key: user.public_key,
        solana_explorer_url: explorer_url,
        phantom_wallet_import_info: phantom_info,
    }))
}

/// POST /buy-token - Transfer tokens from master wallet to user (requires auth)
pub async fn buy_token(
    State(state): State<AppState>,
    claims: Claims,
    Json(req): Json<BuyTokenRequest>,
) -> Result<Json<BuyTokenResponse>, AppError> {
    
    // Convert tokens to amount (internal conversion)
    let amount = if let Some(tokens) = req.tokens {
        // User provided tokens (e.g., 5.0)
        if tokens <= 0.0 {
            return Err(AppError::Validation("Tokens must be greater than 0".to_string()));
        }
        // Convert to raw amount: tokens × 1,000,000,000 (9 decimals)
        (tokens * 1_000_000_000.0) as u64
    } else if let Some(amt) = req.amount {
        // User provided raw amount (backward compatibility)
        if amt == 0 {
            return Err(AppError::Validation("Amount must be greater than 0".to_string()));
        }
        amt
    } else {
        return Err(AppError::Validation(
            "Either 'tokens' or 'amount' must be provided".to_string(),
        ));
    };

    let user = state.db.get_user_by_id(claims.user_id)?;
    let user_pubkey = parse_pubkey(&user.public_key)?;

    // Get master wallet keypair
    let master_keypair = state.solana_service.get_master_keypair()?;

    // Transfer tokens from master to user
    let signature = state
        .solana_service
        .transfer_tokens(&master_keypair, &user_pubkey, amount)
        .await?;

    Ok(Json(BuyTokenResponse {
        message: "Tokens purchased successfully".to_string(),
        transaction_signature: signature,
        amount,
    }))
}

/// POST /transfer - Transfer tokens between users (requires auth)
pub async fn transfer(
    State(state): State<AppState>,
    claims: Claims,
    Json(req): Json<TransferRequest>,
) -> Result<Json<TransferResponse>, AppError> {
    
    // Convert tokens to amount (internal conversion)
    let amount = if let Some(tokens) = req.tokens {
        // User provided tokens (e.g., 5.0)
        if tokens <= 0.0 {
            return Err(AppError::Validation("Tokens must be greater than 0".to_string()));
        }
        // Convert to raw amount: tokens × 1,000,000,000 (9 decimals)
        (tokens * 1_000_000_000.0) as u64
    } else if let Some(amt) = req.amount {
        // User provided raw amount (backward compatibility)
        if amt == 0 {
            return Err(AppError::Validation("Amount must be greater than 0".to_string()));
        }
        amt
    } else {
        return Err(AppError::Validation(
            "Either 'tokens' or 'amount' must be provided".to_string(),
        ));
    };

    let from_user = state.db.get_user_by_id(claims.user_id)?;
    let to_pubkey = parse_pubkey(&req.to)?;

    // Get sender's keypair
    let from_keypair = keypair_from_encrypted(
        &from_user.encrypted_private_key,
        &state.config.encryption_key_bytes(),
    )?;

    // Check balance before transfer
    let from_pubkey = parse_pubkey(&from_user.public_key)?;
    let balance = state.solana_service.get_token_balance(&from_pubkey).await?;
    if balance < amount {
        return Err(AppError::Validation("Insufficient balance".to_string()));
    }

    // Transfer tokens
    let signature = state
        .solana_service
        .transfer_tokens(&from_keypair, &to_pubkey, amount)
        .await?;

    Ok(Json(TransferResponse {
        message: "Transfer successful".to_string(),
        transaction_signature: signature,
        amount,
    }))
}

/// POST /deduct - Deduct tokens from user to master wallet (admin operation)
pub async fn deduct(
    State(state): State<AppState>,
    claims: Claims,
    Json(req): Json<DeductRequest>,
) -> Result<Json<DeductResponse>, AppError> {
    if req.amount == 0 {
        return Err(AppError::Validation("Amount must be greater than 0".to_string()));
    }

    // Determine target user (default to current user if not specified)
    let target_user_id = req.user_id.unwrap_or(claims.user_id);
    let target_user = state.db.get_user_by_id(target_user_id)?;
    let target_pubkey = parse_pubkey(&target_user.public_key)?;

    // Check balance
    let balance = state.solana_service.get_token_balance(&target_pubkey).await?;
    if balance < req.amount {
        return Err(AppError::Validation("Insufficient balance".to_string()));
    }

    // Get target user's keypair
    let target_keypair = keypair_from_encrypted(
        &target_user.encrypted_private_key,
        &state.config.encryption_key_bytes(),
    )?;

    // Get master wallet pubkey
    let master_keypair = state.solana_service.get_master_keypair()?;
    let master_pubkey = master_keypair.pubkey();

    // Transfer tokens from user to master
    let signature = state
        .solana_service
        .transfer_tokens(&target_keypair, &master_pubkey, req.amount)
        .await?;

    Ok(Json(DeductResponse {
        message: "Tokens deducted successfully".to_string(),
        transaction_signature: signature,
        amount: req.amount,
    }))
}

/// POST /reward - Send tokens from master wallet to user (admin operation)
/// ⚠️ ADMIN ONLY - Sirf user_id = 1 (admin) hi call kar sakta hai
pub async fn reward(
    State(state): State<AppState>,
    claims: Claims,
    Json(req): Json<RewardRequest>,
) -> Result<Json<RewardResponse>, AppError> {
    
    // ⚠️ ADMIN CHECK - Sirf user_id = 1 ko allow karo
    if claims.user_id != 1 {
        return Err(AppError::Auth(
            "Admin access required. Only user_id = 1 can send rewards.".to_string()
        ));
    }
    
    // Convert tokens to amount (internal conversion)
    let amount = if let Some(tokens) = req.tokens {
        // User provided tokens (e.g., 5.0)
        if tokens <= 0.0 {
            return Err(AppError::Validation("Tokens must be greater than 0".to_string()));
        }
        // Convert to raw amount: tokens × 1,000,000,000 (9 decimals)
        (tokens * 1_000_000_000.0) as u64
    } else if let Some(amt) = req.amount {
        // User provided raw amount (backward compatibility)
        if amt == 0 {
            return Err(AppError::Validation("Amount must be greater than 0".to_string()));
        }
        amt
    } else {
        return Err(AppError::Validation(
            "Either 'tokens' or 'amount' must be provided".to_string(),
        ));
    };

    let target_user = state.db.get_user_by_id(req.user_id)?;
    let target_pubkey = parse_pubkey(&target_user.public_key)?;

    // Get master wallet keypair
    let master_keypair = state.solana_service.get_master_keypair()?;

    // Transfer tokens from master to user
    let signature = state
        .solana_service
        .transfer_tokens(&master_keypair, &target_pubkey, amount)
        .await?;

    Ok(Json(RewardResponse {
        message: "Reward sent successfully".to_string(),
        transaction_signature: signature,
        amount,
    }))
}

/// POST /send-to-address - Send tokens from master wallet to any Solana address (admin operation)
/// Direct Solana address par tokens bhejne ke liye - database registration nahi chahiye
/// ⚠️ ADMIN ONLY - Sirf user_id = 1 (admin) hi call kar sakta hai
pub async fn send_to_address(
    State(state): State<AppState>,
    claims: Claims,
    Json(req): Json<SendToAddressRequest>,
) -> Result<Json<SendToAddressResponse>, AppError> {
    
    // ⚠️ ADMIN CHECK - Sirf user_id = 1 ko allow karo
    // Production mein yeh zaruri hai, otherwise koi bhi master wallet se tokens bhej sakta hai!
    if claims.user_id != 1 {
        return Err(AppError::Auth(
            "Admin access required. Only user_id = 1 can send tokens to arbitrary addresses.".to_string()
        ));
    }
    
    // Convert tokens to amount (internal conversion)
    let amount = if let Some(tokens) = req.tokens {
        // User provided tokens (e.g., 5.0)
        if tokens <= 0.0 {
            return Err(AppError::Validation("Tokens must be greater than 0".to_string()));
        }
        // Convert to raw amount: tokens × 1,000,000,000 (9 decimals)
        (tokens * 1_000_000_000.0) as u64
    } else if let Some(amt) = req.amount {
        // User provided raw amount (backward compatibility)
        if amt == 0 {
            return Err(AppError::Validation("Amount must be greater than 0".to_string()));
        }
        amt
    } else {
        return Err(AppError::Validation(
            "Either 'tokens' or 'amount' must be provided".to_string(),
        ));
    };

    // Parse target address
    let target_pubkey = parse_pubkey(&req.to_address)?;

    // Get master wallet keypair
    let master_keypair = state.solana_service.get_master_keypair()?;

    // Create ATA if needed for target address
    state
        .solana_service
        .create_associated_token_account_if_needed(&target_pubkey, &master_keypair)
        .await?;

    // Transfer tokens from master to target address
    let signature = state
        .solana_service
        .transfer_tokens(&master_keypair, &target_pubkey, amount)
        .await?;

    // Create explorer URL
    let explorer_url = format!("https://explorer.solana.com/tx/{}", signature);

    Ok(Json(SendToAddressResponse {
        message: "Tokens sent successfully to address".to_string(),
        transaction_signature: signature,
        amount,
        to_address: req.to_address,
        solana_explorer_url: explorer_url,
    }))
}

/// GET /admin/master-balance - Get master wallet balance and details (Admin Only)
/// Master wallet ka complete info - SOL balance, token balance, explorer link
pub async fn get_master_balance(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<MasterBalanceResponse>, AppError> {

    // ⚠️ ADMIN CHECK - Sirf user_id = 1 ko allow karo
    if claims.user_id != 1 {
        return Err(AppError::Auth(
            "Admin access required. Only user_id = 1 can view master wallet balance.".to_string()
        ));
    }

    // Get master wallet keypair
    let master_keypair = state.solana_service.get_master_keypair()?;
    let master_pubkey = master_keypair.pubkey();

    // Get SOL balance (for transaction fees)
    let sol_balance = state
        .solana_service
        .get_sol_balance(&master_pubkey)
        .await?;

    // Convert lamports to SOL (1 SOL = 1,000,000,000 lamports)
    let sol_balance_formatted = sol_balance as f64 / 1_000_000_000.0;

    // Get token balance
    let token_balance = state
        .solana_service
        .get_token_balance(&master_pubkey)
        .await?;

    // Format token balance (9 decimals for KARMM)
    let token_balance_formatted = format!("{:.9}", token_balance as f64 / 1_000_000_000.0);

    // Create explorer URL
    let explorer_url = format!("https://explorer.solana.com/address/{}", master_pubkey);

    // Determine status based on balances
    let status = if sol_balance < 10_000_000 {
        // Less than 0.01 SOL
        "⚠️ WARNING: Low SOL balance - Add SOL for transaction fees!"
    } else if token_balance == 0 {
        "⚠️ WARNING: No KARMM tokens - Add tokens to master wallet!"
    } else {
        "✅ Master wallet is healthy and ready"
    };

    Ok(Json(MasterBalanceResponse {
        message: "Master wallet balance retrieved successfully".to_string(),
        master_public_key: master_pubkey.to_string(),
        sol_balance: sol_balance_formatted,
        token_balance_raw: token_balance,
        token_balance_formatted,
        token_mint: state.config.karmm_mint_address.clone(),
        solana_explorer_url: explorer_url,
        status: status.to_string(),
    }))
}

/// POST /test/transfer-to-master-by-address - Transfer tokens using address (for registered users)
/// Address se transfer - database mein stored private key use hoti hai
pub async fn transfer_to_master_by_address(
    State(state): State<AppState>,
    Json(req): Json<TransferToMasterByAddressRequest>,
) -> Result<Json<TestTransferToMasterResponse>, AppError> {
    if req.amount == 0 {
        return Err(AppError::Validation("Amount must be greater than 0".to_string()));
    }

    // Parse public key from address
    let from_pubkey = parse_pubkey(&req.from_address)?;

    // Find user by public key in database
    let user = state.db.get_user_by_public_key(&req.from_address)?;

    // Decrypt private key from database
    let from_keypair = keypair_from_encrypted(
        &user.encrypted_private_key,
        &state.config.encryption_key_bytes(),
    )?;

    // Check balance before transfer
    let balance = state
        .solana_service
        .get_token_balance(&from_pubkey)
        .await?;

    if balance < req.amount {
        return Err(AppError::Validation(format!(
            "Insufficient balance. Available: {}, Required: {}",
            balance, req.amount
        )));
    }

    // Get master wallet pubkey
    let master_keypair = state.solana_service.get_master_keypair()?;
    let master_pubkey = master_keypair.pubkey();

    // Transfer tokens from source wallet to master wallet
    let signature = state
        .solana_service
        .transfer_tokens(&from_keypair, &master_pubkey, req.amount)
        .await?;

    Ok(Json(TestTransferToMasterResponse {
        message: "Tokens transferred to master wallet successfully".to_string(),
        transaction_signature: signature,
        amount: req.amount,
        from_address: from_pubkey.to_string(),
        to_address: master_pubkey.to_string(),
    }))
}

/// POST /test/transfer-to-master - Transfer tokens from any wallet to master wallet (Testing)
/// ⚠️ WARNING: This is for testing only! In production, remove or secure this endpoint.
pub async fn test_transfer_to_master(
    State(state): State<AppState>,
    Json(req): Json<TestTransferToMasterRequest>,
) -> Result<Json<TestTransferToMasterResponse>, AppError> {
    if req.amount == 0 {
        return Err(AppError::Validation("Amount must be greater than 0".to_string()));
    }

    // Validate private key format (128 hex characters)
    if req.from_private_key.len() != 128 {
        return Err(AppError::Validation(
            "Private key must be exactly 128 hex characters".to_string(),
        ));
    }

    // Parse private key from hex
    let private_key_bytes = hex::decode(&req.from_private_key)
        .map_err(|e| AppError::Validation(format!("Invalid hex format: {}", e)))?;

    if private_key_bytes.len() != 64 {
        return Err(AppError::Validation(
            "Private key must be 64 bytes".to_string(),
        ));
    }

    // Create keypair from private key
    let from_keypair = solana_sdk::signature::Keypair::from_bytes(&private_key_bytes)
        .map_err(|e| AppError::Validation(format!("Invalid private key: {}", e)))?;

    let from_pubkey = from_keypair.pubkey();

    // Check balance before transfer
    let balance = state
        .solana_service
        .get_token_balance(&from_pubkey)
        .await?;

    if balance < req.amount {
        return Err(AppError::Validation(format!(
            "Insufficient balance. Available: {}, Required: {}",
            balance, req.amount
        )));
    }

    // Get master wallet pubkey
    let master_keypair = state.solana_service.get_master_keypair()?;
    let master_pubkey = master_keypair.pubkey();

    // Transfer tokens from source wallet to master wallet
    let signature = state
        .solana_service
        .transfer_tokens(&from_keypair, &master_pubkey, req.amount)
        .await?;

    Ok(Json(TestTransferToMasterResponse {
        message: "Tokens transferred to master wallet successfully".to_string(),
        transaction_signature: signature,
        amount: req.amount,
        from_address: from_pubkey.to_string(),
        to_address: master_pubkey.to_string(),
    }))
}

/// JWT authentication middleware
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Auth("Missing Authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Auth("Invalid Authorization header format".to_string()));
    }

    let token = &auth_header[7..];
    let claims = crate::auth::verify_token(token, &state.config.jwt_secret)?;

    // Attach claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Create router with all routes
pub fn create_router(state: AppState) -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login));

    // Testing routes (public - for testing only)
    // ⚠️ WARNING: Remove or secure in production!
    let test_routes = Router::new()
        .route("/test/transfer-to-master", post(test_transfer_to_master))
        .route("/test/transfer-to-master-by-address", post(transfer_to_master_by_address));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/balance", get(get_balance))
        .route("/buy-token", post(buy_token))
        .route("/transfer", post(transfer))
        .route("/deduct", post(deduct))
        .route("/reward", post(reward))
        .route("/send-to-address", post(send_to_address))
        .route("/admin/master-balance", get(get_master_balance))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(test_routes)
        .merge(protected_routes)
        .with_state(state)
}
