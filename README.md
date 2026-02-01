# Solana Token Backend System

A production-ready custodial wallet system for managing SPL tokens on **Solana MAINNET**, built with Rust and Axum.

## ⚠️ IMPORTANT: This is REAL Mainnet!

- ✅ **Real Solana Mainnet** - Not devnet or testnet
- ✅ **Real SPL Tokens** - Actual blockchain transactions
- ✅ **Viewable in Phantom/Solflare** - Your tokens will appear in real wallet apps
- ✅ **Production Ready** - Ready for real users and real transactions

## Features

- 🔐 **Custodial Wallet System**: Automatically creates Solana wallets for users
- 🔒 **Secure Encryption**: Private keys encrypted with AES-256-GCM before storage
- 🎫 **JWT Authentication**: Secure token-based authentication
- 💰 **SPL Token Management**: Full support for SPL token operations
- ⛓️ **Solana Mainnet**: Production-ready integration with Solana blockchain
- 🗄️ **SQLite Database**: Lightweight database for user data
- 🚀 **RESTful API**: Clean, well-documented API endpoints

## Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── db.rs                # Database operations
├── auth.rs              # JWT authentication
├── encryption.rs        # Private key encryption/decryption
├── wallet.rs            # Wallet creation and management
├── solana_service.rs    # Solana blockchain operations
├── routes.rs            # API route handlers
└── errors.rs            # Error handling
```

## Prerequisites

- Rust 1.70+ installed ([rustup.rs](https://rustup.rs/))
- Solana CLI tools (optional, for key generation)
- Helius or Alchemy RPC API key for Solana mainnet
- Your KARMM token mint address on Solana mainnet

## Setup Instructions

### 1. Clone and Navigate

```bash
cd Solana_rust
```

### 2. Install Dependencies

```bash
cargo build
```

### 3. Configure Environment Variables

Copy the example environment file:

```bash
cp .env.example .env
```

Edit `.env` and fill in all required values:

```env
# Generate encryption key (32 bytes hex)
openssl rand -hex 32

# Generate JWT secret (use a strong random string)
openssl rand -base64 32

# Set your Solana RPC URL (Helius or Alchemy)
SOLANA_RPC_URL=https://mainnet.helius-rpc.com/?api-key=YOUR_API_KEY

# Set your KARMM token mint address
KARMM_MINT_ADDRESS=YourKarmmMintAddressHere

# Generate master wallet private key
solana-keygen new --outfile master-wallet.json --no-bip39-passphrase
# Convert to hex (you'll need to extract the secret key array)
```

**Important Security Notes:**
- Never commit `.env` file to version control
- Use strong, random values for `JWT_SECRET` and `ENCRYPTION_KEY`
- Keep `MASTER_WALLET_PRIVATE_KEY` secure - it controls your treasury wallet
- Use a private RPC endpoint (Helius/Alchemy) for production

### 4. Generate Master Wallet (if needed)

If you need to create a new master wallet:

```bash
solana-keygen new --outfile master-wallet.json --no-bip39-passphrase
```

Extract the private key bytes and convert to hex format for the `.env` file.

### 5. Run the Server

```bash
cargo run
```

The server will start on `http://localhost:3000` (or your configured `SERVER_PORT`).

## API Endpoints

### Public Endpoints

#### POST `/signup`
Create a new user account and Solana wallet.

**Request:**
```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response:**
```json
{
  "message": "User created successfully",
  "user_id": 1,
  "public_key": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

#### POST `/login`
Authenticate user and get JWT token.

**Request:**
```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response:**
```json
{
  "message": "Login successful",
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user_id": 1
}
```

### Protected Endpoints (Require JWT Token)

All protected endpoints require the `Authorization: Bearer <token>` header.

#### GET `/balance`
Get user's KARMM token balance from Solana blockchain.

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "balance": 1000000000,
  "public_key": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"
}
```

#### POST `/buy-token`
Transfer tokens from master wallet to user (after payment confirmation).

**Request:**
```json
{
  "amount": 1000000000,
  "payment_id": "payment_12345"
}
```

**Response:**
```json
{
  "message": "Tokens purchased successfully",
  "transaction_signature": "5VERv8NMvzbJMEkV8xkRDZ18YVp53vVvAuoKvfONSWv5uGUQYFpYMDyRviWNjyw5VFxOGewYJby",
  "amount": 1000000000
}
```

#### POST `/transfer`
Transfer tokens between users.

**Request:**
```json
{
  "to": "RecipientPublicKeyHere",
  "amount": 500000000
}
```

**Response:**
```json
{
  "message": "Transfer successful",
  "transaction_signature": "5VERv8NMvzbJMEkV8xkRDZ18YVp53vVvAuoKvfONSWv5uGUQYFpYMDyRviWNjyw5VFxOGewYJby",
  "amount": 500000000
}
```

#### POST `/deduct`
Deduct tokens from user back to master wallet.

**Request:**
```json
{
  "amount": 100000000,
  "user_id": 1  // Optional, defaults to current user
}
```

**Response:**
```json
{
  "message": "Tokens deducted successfully",
  "transaction_signature": "5VERv8NMvzbJMEkV8xkRDZ18YVp53vVvAuoKvfONSWv5uGUQYFpYMDyRviWNjyw5VFxOGewYJby",
  "amount": 100000000
}
```

#### POST `/reward`
Send tokens from master wallet to user as reward.

**Request:**
```json
{
  "user_id": 1,
  "amount": 2000000000
}
```

**Response:**
```json
{
  "message": "Reward sent successfully",
  "transaction_signature": "5VERv8NMvzbJMEkV8xkRDZ18YVp53vVvAuoKvfONSWv5uGUQYFpYMDyRviWNjyw5VFxOGewYJby",
  "amount": 2000000000
}
```

## Example Usage

See `examples/curl_requests.sh` for complete cURL examples.

Quick test:

```bash
# Signup
curl -X POST http://localhost:3000/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123"}'

# Login (save the token)
TOKEN=$(curl -s -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123"}' | jq -r '.token')

# Get balance
curl -X GET http://localhost:3000/balance \
  -H "Authorization: Bearer $TOKEN"
```

## Security Considerations

1. **Private Key Encryption**: All private keys are encrypted with AES-256-GCM before storage
2. **Password Hashing**: Passwords are hashed using bcrypt
3. **JWT Tokens**: Secure token-based authentication with expiration
4. **Environment Variables**: Sensitive data stored in `.env` (never commit)
5. **Rate Limiting**: Consider adding rate limiting middleware for production
6. **HTTPS**: Always use HTTPS in production
7. **Master Wallet Security**: Keep master wallet private key extremely secure

## Token Amounts

SPL tokens typically use 9 decimals. When sending amounts:
- `1000000000` = 1.0 token (with 9 decimals)
- `100000000` = 0.1 token
- `1` = 0.000000001 token

Adjust the `decimals` value in `solana_service.rs` if your token uses different decimals.

## Database Schema

The system creates two tables:

- **users**: Stores user accounts, encrypted private keys, and public keys
- **master_wallet**: Stores master wallet information (optional, currently uses env var)

## Error Handling

All endpoints return consistent error responses:

```json
{
  "error": "Error message here",
  "status": 400
}
```

Common status codes:
- `200`: Success
- `400`: Bad Request (validation errors)
- `401`: Unauthorized (authentication required)
- `500`: Internal Server Error

## Production Deployment

1. **Environment**: Set all environment variables securely
2. **Database**: Consider migrating to PostgreSQL for production
3. **Rate Limiting**: Add rate limiting middleware
4. **Logging**: Add structured logging (e.g., `tracing`)
5. **Monitoring**: Add health check endpoints
6. **Backup**: Regular database backups
7. **SSL/TLS**: Use HTTPS with valid certificates
8. **Master Wallet**: Use hardware wallet or secure key management service

## Troubleshooting

### "Failed to create token account"
- Ensure master wallet has enough SOL for transaction fees
- Check RPC endpoint is accessible
- Verify mint address is correct

### "Insufficient balance"
- Check user's token balance on Solana explorer
- Ensure associated token account exists

### "Invalid public key"
- Verify Solana public key format (base58 encoded)
- Check key length (should be 32 bytes)

## License

MIT License - See LICENSE file for details

## Support

For issues and questions, please open an issue on GitHub.
