# Complete API Flow Test Script
# Step-by-step testing of Signup → Login → Transfer

Write-Host "========================================"
Write-Host "Complete API Flow Test"
Write-Host "========================================"
Write-Host ""

$baseUrl = "http://localhost:3000"

# Step 1: Signup User 1
Write-Host "Step 1: Signing up User 1..."
try {
    $user1 = Invoke-RestMethod -Uri "$baseUrl/signup" `
        -Method POST `
        -ContentType "application/json" `
        -Body '{"email":"user1@test.com","password":"password123"}'

    Write-Host "✅ User 1 Created"
    Write-Host "   User ID: $($user1.user_id)"
    Write-Host "   Public Key: $($user1.public_key)"
    $user1Token = $user1.token
    $user1Id = $user1.user_id
    $user1PublicKey = $user1.public_key
    Write-Host ""
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    exit
}

# Step 2: Signup User 2
Write-Host "Step 2: Signing up User 2..."
try {
    $user2 = Invoke-RestMethod -Uri "$baseUrl/signup" `
        -Method POST `
        -ContentType "application/json" `
        -Body '{"email":"user2@test.com","password":"password123"}'

    Write-Host "✅ User 2 Created"
    Write-Host "   User ID: $($user2.user_id)"
    Write-Host "   Public Key: $($user2.public_key)"
    $user2Id = $user2.user_id
    $user2PublicKey = $user2.public_key
    Write-Host ""
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    exit
}

# Step 3: Login User 1 (Admin)
Write-Host "Step 3: Logging in User 1..."
try {
    $login1 = Invoke-RestMethod -Uri "$baseUrl/login" `
        -Method POST `
        -ContentType "application/json" `
        -Body '{"email":"user1@test.com","password":"password123"}'

    $adminToken = $login1.token
    Write-Host "✅ User 1 Logged In"
    Write-Host "   Token received (length: $($adminToken.Length))"
    Write-Host ""
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    exit
}

# Step 4: Check User 1 Balance (Should be 0)
Write-Host "Step 4: Checking User 1 Balance..."
try {
    $balance1 = Invoke-RestMethod -Uri "$baseUrl/balance" `
        -Method GET `
        -Headers @{"Authorization"="Bearer $adminToken"}

    Write-Host "✅ Balance Checked"
    Write-Host "   Balance: $($balance1.balance_formatted) KARMM"
    Write-Host "   Public Key: $($balance1.public_key)"
    Write-Host ""
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    exit
}

# Step 5: Master Wallet Se User 1 Ko 10 Tokens Send
Write-Host "Step 5: Sending 10 tokens from Master Wallet to User 1..."
try {
    $reward1 = Invoke-RestMethod -Uri "$baseUrl/reward" `
        -Method POST `
        -Headers @{"Authorization"="Bearer $adminToken"} `
        -ContentType "application/json" `
        -Body (@{
            user_id = $user1Id
            tokens = 10.0
        } | ConvertTo-Json)

    Write-Host "✅ Tokens Sent Successfully"
    Write-Host "   Transaction: $($reward1.transaction_signature)"
    Write-Host "   Amount: $($reward1.amount)"
    Write-Host "   Verify: https://explorer.solana.com/tx/$($reward1.transaction_signature)"
    Write-Host ""
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    Write-Host "   Make sure master wallet has tokens and SOL for fees"
    exit
}

# Step 6: Master Wallet Se User 2 Ko 5 Tokens Send
Write-Host "Step 6: Sending 5 tokens from Master Wallet to User 2..."
try {
    $reward2 = Invoke-RestMethod -Uri "$baseUrl/reward" `
        -Method POST `
        -Headers @{"Authorization"="Bearer $adminToken"} `
        -ContentType "application/json" `
        -Body (@{
            user_id = $user2Id
            tokens = 5.0
        } | ConvertTo-Json)

    Write-Host "✅ Tokens Sent Successfully"
    Write-Host "   Transaction: $($reward2.transaction_signature)"
    Write-Host "   Amount: $($reward2.amount)"
    Write-Host ""
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    exit
}

# Step 7: User 1 Balance Check (Should be 10)
Write-Host "Step 7: Checking User 1 Balance (after reward)..."
try {
    Start-Sleep -Seconds 2  # Wait for transaction confirmation
    $balance1After = Invoke-RestMethod -Uri "$baseUrl/balance" `
        -Method GET `
        -Headers @{"Authorization"="Bearer $adminToken"}

    Write-Host "✅ Balance Checked"
    Write-Host "   Balance: $($balance1After.balance_formatted) KARMM"
    Write-Host ""
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    exit
}

# Step 8: User 2 Login
Write-Host "Step 8: Logging in User 2..."
try {
    $login2 = Invoke-RestMethod -Uri "$baseUrl/login" `
        -Method POST `
        -ContentType "application/json" `
        -Body '{"email":"user2@test.com","password":"password123"}'

    $user2Token = $login2.token
    Write-Host "✅ User 2 Logged In"
    Write-Host ""
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    exit
}

# Step 9: User 2 Balance Check (Should be 5)
Write-Host "Step 9: Checking User 2 Balance..."
try {
    $balance2 = Invoke-RestMethod -Uri "$baseUrl/balance" `
        -Method GET `
        -Headers @{"Authorization"="Bearer $user2Token"}

    Write-Host "✅ Balance Checked"
    Write-Host "   Balance: $($balance2.balance_formatted) KARMM"
    Write-Host ""
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    exit
}

# Step 10: User 1 Se User 2 Ko 3 Tokens Transfer
Write-Host "Step 10: Transferring 3 tokens from User 1 to User 2..."
try {
    $transfer = Invoke-RestMethod -Uri "$baseUrl/transfer" `
        -Method POST `
        -Headers @{"Authorization"="Bearer $adminToken"} `
        -ContentType "application/json" `
        -Body (@{
            to = $user2PublicKey
            tokens = 3.0
        } | ConvertTo-Json)

    Write-Host "✅ Transfer Complete"
    Write-Host "   Transaction: $($transfer.transaction_signature)"
    Write-Host "   Amount: $($transfer.amount)"
    Write-Host "   Verify: https://explorer.solana.com/tx/$($transfer.transaction_signature)"
    Write-Host ""
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    exit
}

# Step 11: Final Balance Check
Write-Host "Step 11: Final Balance Check..."
Write-Host ""

try {
    Start-Sleep -Seconds 2  # Wait for transaction confirmation
    
    $finalBalance1 = Invoke-RestMethod -Uri "$baseUrl/balance" `
        -Method GET `
        -Headers @{"Authorization"="Bearer $adminToken"}

    $finalBalance2 = Invoke-RestMethod -Uri "$baseUrl/balance" `
        -Method GET `
        -Headers @{"Authorization"="Bearer $user2Token"}

    Write-Host "========================================"
    Write-Host "Final Results"
    Write-Host "========================================"
    Write-Host "User 1 Balance: $($finalBalance1.balance_formatted) KARMM"
    Write-Host "   Expected: 7.0 (10 - 3)"
    Write-Host ""
    Write-Host "User 2 Balance: $($finalBalance2.balance_formatted) KARMM"
    Write-Host "   Expected: 8.0 (5 + 3)"
    Write-Host ""
    Write-Host "✅ Complete Flow Test Finished!"
    Write-Host ""
    Write-Host "User 1 Explorer: $($finalBalance1.solana_explorer_url)"
    Write-Host "User 2 Explorer: $($finalBalance2.solana_explorer_url)"
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
}
