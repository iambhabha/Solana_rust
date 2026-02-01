#!/bin/bash

# Solana Token Backend API - Example cURL Requests
# Make sure to replace BASE_URL and TOKEN with actual values

BASE_URL="http://localhost:3000"
TOKEN=""  # Will be set after login/signup

echo "=== 1. User Signup ==="
echo "Creates a new user account and Solana wallet"
echo ""
curl -X POST "${BASE_URL}/signup" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securepassword123"
  }' | jq '.'

echo -e "\n\n=== 2. User Login ==="
echo "Authenticate and get JWT token"
echo ""
RESPONSE=$(curl -s -X POST "${BASE_URL}/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securepassword123"
  }')

TOKEN=$(echo $RESPONSE | jq -r '.token')
echo $RESPONSE | jq '.'

echo -e "\n\n=== 3. Get Token Balance ==="
echo "Fetch user's KARMM token balance from Solana blockchain"
echo ""
curl -X GET "${BASE_URL}/balance" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Content-Type: application/json" | jq '.'

echo -e "\n\n=== 4. Buy Tokens ==="
echo "Transfer tokens from master wallet to user (after payment)"
echo ""
curl -X POST "${BASE_URL}/buy-token" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 1000000000,
    "payment_id": "payment_12345"
  }' | jq '.'

echo -e "\n\n=== 5. Transfer Tokens ==="
echo "Transfer tokens from current user to another user"
echo "Replace RECIPIENT_PUBKEY with actual recipient public key"
echo ""
RECIPIENT_PUBKEY="RecipientSolanaPublicKeyHere"
curl -X POST "${BASE_URL}/transfer" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Content-Type: application/json" \
  -d "{
    \"to\": \"${RECIPIENT_PUBKEY}\",
    \"amount\": 500000000
  }" | jq '.'

echo -e "\n\n=== 6. Deduct Tokens ==="
echo "Deduct tokens from user back to master wallet"
echo ""
curl -X POST "${BASE_URL}/deduct" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 100000000
  }' | jq '.'

echo -e "\n\n=== 7. Reward Tokens ==="
echo "Send tokens from master wallet to user as reward"
echo "Replace USER_ID with actual user ID"
echo ""
USER_ID=1
curl -X POST "${BASE_URL}/reward" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Content-Type: application/json" \
  -d "{
    \"user_id\": ${USER_ID},
    \"amount\": 2000000000
  }" | jq '.'

echo -e "\n\n=== All requests completed! ==="
