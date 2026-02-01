#!/bin/bash

# Complete API Flow Test Script (Bash/cURL)
# Step-by-step testing of Signup → Login → Transfer

BASE_URL="http://localhost:3000"

echo "========================================"
echo "Complete API Flow Test"
echo "========================================"
echo ""

# Step 1: Signup User 1
echo "Step 1: Signing up User 1..."
USER1_RESPONSE=$(curl -s -X POST "${BASE_URL}/signup" \
  -H "Content-Type: application/json" \
  -d '{"email":"user1@test.com","password":"password123"}')

USER1_ID=$(echo $USER1_RESPONSE | jq -r '.user_id')
USER1_PUBLIC_KEY=$(echo $USER1_RESPONSE | jq -r '.public_key')
USER1_TOKEN=$(echo $USER1_RESPONSE | jq -r '.token')

echo "✅ User 1 Created"
echo "   User ID: $USER1_ID"
echo "   Public Key: $USER1_PUBLIC_KEY"
echo ""

# Step 2: Signup User 2
echo "Step 2: Signing up User 2..."
USER2_RESPONSE=$(curl -s -X POST "${BASE_URL}/signup" \
  -H "Content-Type: application/json" \
  -d '{"email":"user2@test.com","password":"password123"}')

USER2_ID=$(echo $USER2_RESPONSE | jq -r '.user_id')
USER2_PUBLIC_KEY=$(echo $USER2_RESPONSE | jq -r '.public_key')

echo "✅ User 2 Created"
echo "   User ID: $USER2_ID"
echo "   Public Key: $USER2_PUBLIC_KEY"
echo ""

# Step 3: Login User 1 (Admin)
echo "Step 3: Logging in User 1..."
LOGIN1_RESPONSE=$(curl -s -X POST "${BASE_URL}/login" \
  -H "Content-Type: application/json" \
  -d '{"email":"user1@test.com","password":"password123"}')

ADMIN_TOKEN=$(echo $LOGIN1_RESPONSE | jq -r '.token')

echo "✅ User 1 Logged In"
echo ""

# Step 4: Check User 1 Balance
echo "Step 4: Checking User 1 Balance..."
BALANCE1=$(curl -s -X GET "${BASE_URL}/balance" \
  -H "Authorization: Bearer ${ADMIN_TOKEN}")

BALANCE1_FORMATTED=$(echo $BALANCE1 | jq -r '.balance_formatted')
echo "✅ Balance: $BALANCE1_FORMATTED KARMM"
echo ""

# Step 5: Master Wallet Se User 1 Ko 10 Tokens Send
echo "Step 5: Sending 10 tokens from Master Wallet to User 1..."
REWARD1=$(curl -s -X POST "${BASE_URL}/reward" \
  -H "Authorization: Bearer ${ADMIN_TOKEN}" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\":${USER1_ID},\"tokens\":10.0}")

TX1=$(echo $REWARD1 | jq -r '.transaction_signature')
echo "✅ Tokens Sent"
echo "   Transaction: $TX1"
echo "   Verify: https://explorer.solana.com/tx/$TX1"
echo ""

# Step 6: Master Wallet Se User 2 Ko 5 Tokens Send
echo "Step 6: Sending 5 tokens from Master Wallet to User 2..."
REWARD2=$(curl -s -X POST "${BASE_URL}/reward" \
  -H "Authorization: Bearer ${ADMIN_TOKEN}" \
  -H "Content-Type: application/json" \
  -d "{\"user_id\":${USER2_ID},\"tokens\":5.0}")

TX2=$(echo $REWARD2 | jq -r '.transaction_signature')
echo "✅ Tokens Sent"
echo "   Transaction: $TX2"
echo ""

# Step 7: User 1 Balance Check (After Reward)
echo "Step 7: Checking User 1 Balance (after reward)..."
sleep 2
BALANCE1_AFTER=$(curl -s -X GET "${BASE_URL}/balance" \
  -H "Authorization: Bearer ${ADMIN_TOKEN}")

BALANCE1_AFTER_FORMATTED=$(echo $BALANCE1_AFTER | jq -r '.balance_formatted')
echo "✅ Balance: $BALANCE1_AFTER_FORMATTED KARMM"
echo ""

# Step 8: User 2 Login
echo "Step 8: Logging in User 2..."
LOGIN2_RESPONSE=$(curl -s -X POST "${BASE_URL}/login" \
  -H "Content-Type: application/json" \
  -d '{"email":"user2@test.com","password":"password123"}')

USER2_TOKEN=$(echo $LOGIN2_RESPONSE | jq -r '.token')
echo "✅ User 2 Logged In"
echo ""

# Step 9: User 2 Balance Check
echo "Step 9: Checking User 2 Balance..."
BALANCE2=$(curl -s -X GET "${BASE_URL}/balance" \
  -H "Authorization: Bearer ${USER2_TOKEN}")

BALANCE2_FORMATTED=$(echo $BALANCE2 | jq -r '.balance_formatted')
echo "✅ Balance: $BALANCE2_FORMATTED KARMM"
echo ""

# Step 10: User 1 Se User 2 Ko 3 Tokens Transfer
echo "Step 10: Transferring 3 tokens from User 1 to User 2..."
TRANSFER=$(curl -s -X POST "${BASE_URL}/transfer" \
  -H "Authorization: Bearer ${ADMIN_TOKEN}" \
  -H "Content-Type: application/json" \
  -d "{\"to\":\"${USER2_PUBLIC_KEY}\",\"tokens\":3.0}")

TX3=$(echo $TRANSFER | jq -r '.transaction_signature')
echo "✅ Transfer Complete"
echo "   Transaction: $TX3"
echo "   Verify: https://explorer.solana.com/tx/$TX3"
echo ""

# Step 11: Final Balance Check
echo "Step 11: Final Balance Check..."
sleep 2

FINAL_BALANCE1=$(curl -s -X GET "${BASE_URL}/balance" \
  -H "Authorization: Bearer ${ADMIN_TOKEN}")

FINAL_BALANCE2=$(curl -s -X GET "${BASE_URL}/balance" \
  -H "Authorization: Bearer ${USER2_TOKEN}")

FINAL_BALANCE1_FORMATTED=$(echo $FINAL_BALANCE1 | jq -r '.balance_formatted')
FINAL_BALANCE2_FORMATTED=$(echo $FINAL_BALANCE2 | jq -r '.balance_formatted')

echo "========================================"
echo "Final Results"
echo "========================================"
echo "User 1 Balance: $FINAL_BALANCE1_FORMATTED KARMM (Expected: 7.0)"
echo "User 2 Balance: $FINAL_BALANCE2_FORMATTED KARMM (Expected: 8.0)"
echo ""
echo "✅ Complete Flow Test Finished!"
