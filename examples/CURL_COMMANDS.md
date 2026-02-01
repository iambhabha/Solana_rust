# Complete cURL Commands - Solana Token Backend

## 🔐 1. Authentication

### Signup (Create User)
```bash
curl -X POST http://localhost:3000/signup \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

**Response:**
```json
{
  "message": "User created successfully",
  "user_id": 1,
  "token": "eyJhbGc...",
  "public_key": "DPBSNAe...",
  "solana_explorer_url": "https://explorer.solana.com/address/...",
  "phantom_wallet_info": "..."
}
```

---

### Login
```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

**Response:**
```json
{
  "message": "Login successful",
  "token": "eyJhbGc...",
  "user_id": 1
}
```

**💡 Token save kar lo - baaki APIs mein use hoga!**

---

## 💰 2. Token Operations

### Get Balance
```bash
curl -X GET http://localhost:3000/balance \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

**Response:**
```json
{
  "balance": 5000000000,
  "balance_formatted": "5.000000000",
  "public_key": "DPBSNAe...",
  "solana_explorer_url": "https://explorer.solana.com/address/...",
  "phantom_wallet_import_info": "..."
}
```

---

### Transfer to Address (User to Any Address)
```bash
curl -X POST http://localhost:3000/transfer \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{
    "to": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "tokens": 5.0
  }'
```

**Response:**
```json
{
  "message": "Transfer successful",
  "transaction_signature": "5KBmW...",
  "amount": 5000000000
}
```

---

### Buy Token
```bash
curl -X POST http://localhost:3000/buy-token \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{
    "tokens": 10.0
  }'
```

---

## 🔒 3. Admin Operations

### Admin Login
```bash
# First user (user_id = 1) is automatically admin
curl -X POST http://localhost:3000/signup \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "admin123"
  }'

# Or login if already exists
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "admin123"
  }'
```

---

### Reward - Send to Registered User (Admin Only)
```bash
curl -X POST http://localhost:3000/reward \
  -H "Authorization: Bearer ADMIN_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": 2,
    "tokens": 100.0
  }'
```

**Response:**
```json
{
  "message": "Reward sent successfully",
  "transaction_signature": "5KBmW...",
  "amount": 100000000000
}
```

**⚠️ Admin only (user_id = 1)**

---

### Send to Any Address (Admin Only) 🆕
```bash
curl -X POST http://localhost:3000/send-to-address \
  -H "Authorization: Bearer ADMIN_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{
    "to_address": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "tokens": 10.0
  }'
```

**Response:**
```json
{
  "message": "Tokens sent successfully to address",
  "transaction_signature": "5KBmW...",
  "amount": 10000000000,
  "to_address": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
  "solana_explorer_url": "https://explorer.solana.com/tx/5KBmW..."
}
```

**⚠️ Admin only - Master wallet se kisi bhi address par bhejo!**

---

### Deduct Tokens
```bash
curl -X POST http://localhost:3000/deduct \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": 2,
    "amount": 1000000000
  }'
```

---

## 🧪 4. Testing Endpoints

### Test Transfer to Master (Using Private Key)
```bash
curl -X POST http://localhost:3000/test/transfer-to-master \
  -H "Content-Type: application/json" \
  -d '{
    "from_private_key": "425082a842cce9ff262d7fe25fd7330f853646a4d3aeda54c093f0fc181a6bb0b7fad28b5584d68034ebd62372e4a8b5f69c48c55c0a18c55d170bf54bf55dce",
    "amount": 1000000000
  }'
```

---

### Test Transfer by Address
```bash
curl -X POST http://localhost:3000/test/transfer-to-master-by-address \
  -H "Content-Type: application/json" \
  -d '{
    "from_address": "DPBSNAeWZFCeGnkrLLFkzVuFFJkKipcc1UcEEyStrmRw",
    "amount": 1000000000
  }'
```

---

## 📝 Complete Flow Example

### Step-by-Step: Admin Sends Tokens to Customer

```bash
# Step 1: Admin Signup/Login
TOKEN=$(curl -s -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@example.com","password":"admin123"}' \
  | jq -r '.token')

echo "Admin Token: $TOKEN"

# Step 2: Check Admin User ID (should be 1)
USER_ID=$(curl -s -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@example.com","password":"admin123"}' \
  | jq -r '.user_id')

echo "Admin User ID: $USER_ID"

# Step 3: Send tokens to customer's Phantom wallet
curl -X POST http://localhost:3000/send-to-address \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "to_address": "CustomerPhantomWalletAddress123...",
    "tokens": 50.0
  }'
```

---

## 🎯 User-to-User Transfer Example

```bash
# Step 1: User 1 Login
TOKEN_1=$(curl -s -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"email":"alice@example.com","password":"alice123"}' \
  | jq -r '.token')

# Step 2: Get User 2's Address (from signup response)
USER2_ADDRESS="BobsWalletAddress123..."

# Step 3: User 1 sends to User 2
curl -X POST http://localhost:3000/transfer \
  -H "Authorization: Bearer $TOKEN_1" \
  -H "Content-Type: application/json" \
  -d "{
    \"to\": \"$USER2_ADDRESS\",
    \"tokens\": 5.0
  }"
```

---

## 🔍 Verify Transaction

```bash
# Get transaction signature from response
TX_SIG="5KBmW123abc..."

# View on Explorer
echo "https://explorer.solana.com/tx/$TX_SIG"
```

---

## 💡 Token Amount Format

### User-Friendly (Recommended):
```json
{
  "tokens": 5.0    // 5 KARMM tokens
}
```

### Raw Format:
```json
{
  "amount": 5000000000   // Same as 5 tokens (9 decimals)
}
```

---

## 📊 Quick Reference

| Endpoint | Method | Auth | Admin? | Purpose |
|----------|--------|------|--------|---------|
| `/signup` | POST | ❌ | ❌ | Create user + wallet |
| `/login` | POST | ❌ | ❌ | Get JWT token |
| `/balance` | GET | ✅ | ❌ | Check balance |
| `/transfer` | POST | ✅ | ❌ | User to address |
| `/buy-token` | POST | ✅ | ❌ | Buy tokens |
| `/reward` | POST | ✅ | ✅ | Master to user (user_id) |
| `/send-to-address` | POST | ✅ | ✅ | Master to any address |
| `/deduct` | POST | ✅ | ❌ | User to master |

---

## ⚠️ Important Notes

1. **Token Format:** Use `Bearer YOUR_TOKEN` in Authorization header
2. **Admin:** Only user_id = 1 can use admin APIs
3. **Amounts:** Use `tokens` field (user-friendly) instead of `amount`
4. **Verify:** Check transactions on Solana Explorer

---

## 🛠️ Troubleshooting

### Error: "Unauthorized"
```bash
# Token expired - login again
TOKEN=$(curl -s -X POST http://localhost:3000/login ... | jq -r '.token')
```

### Error: "Admin access required"
```bash
# Make sure you're using admin token (user_id = 1)
# Check user_id from login response
```

### Error: "Transaction simulation failed"
```bash
# Master wallet needs SOL
cargo run --bin check_wallet
```

---

## ✅ Complete Testing Script

```bash
#!/bin/bash

BASE_URL="http://localhost:3000"

# 1. Admin Signup
echo "1. Creating admin..."
ADMIN=$(curl -s -X POST $BASE_URL/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@example.com","password":"admin123"}')

ADMIN_TOKEN=$(echo $ADMIN | jq -r '.token')
echo "Admin Token: $ADMIN_TOKEN"

# 2. User Signup
echo "2. Creating user..."
USER=$(curl -s -X POST $BASE_URL/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"user123"}')

USER_ID=$(echo $USER | jq -r '.user_id')
USER_ADDRESS=$(echo $USER | jq -r '.public_key')
echo "User ID: $USER_ID"
echo "User Address: $USER_ADDRESS"

# 3. Admin sends tokens to user
echo "3. Admin sending tokens..."
curl -X POST $BASE_URL/reward \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\":$USER_ID,\"tokens\":10.0}"

# 4. Check balance
echo "4. Checking user balance..."
USER_TOKEN=$(echo $USER | jq -r '.token')
curl -X GET $BASE_URL/balance \
  -H "Authorization: Bearer $USER_TOKEN"

echo "✅ Complete!"
```

---

**Save as `test_api.sh` and run:**
```bash
chmod +x test_api.sh
./test_api.sh
```

---

**All APIs ready to use!** 🚀
