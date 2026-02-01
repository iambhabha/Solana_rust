# Admin Flow - Master Wallet se User ko KARMM tokens bhejne ke liye
# Usage: .\admin_send_tokens.ps1

$baseUrl = "http://localhost:3000"

Write-Host "🔐 Step 1: Admin Login kar rahe hain..." -ForegroundColor Cyan
Write-Host ""

# Admin signup/login (pehli baar)
try {
    $signupResponse = Invoke-RestMethod -Uri "$baseUrl/signup" `
        -Method POST `
        -ContentType "application/json" `
        -Body '{"email":"admin@example.com","password":"admin123"}' `
        -ErrorAction Stop
    
    Write-Host "✅ Admin user created successfully!" -ForegroundColor Green
    $token = $signupResponse.token
    $adminUserId = $signupResponse.user_id
    Write-Host "   Admin User ID: $adminUserId" -ForegroundColor White
    Write-Host "   Admin Public Key: $($signupResponse.public_key)" -ForegroundColor White
} catch {
    # Agar already exists, toh login karo
    Write-Host "⚠️  Admin already exists, logging in..." -ForegroundColor Yellow
    
    $loginResponse = Invoke-RestMethod -Uri "$baseUrl/login" `
        -Method POST `
        -ContentType "application/json" `
        -Body '{"email":"admin@example.com","password":"admin123"}'
    
    $token = $loginResponse.token
    $adminUserId = $loginResponse.user_id
    Write-Host "✅ Admin logged in successfully!" -ForegroundColor Green
    Write-Host "   Admin User ID: $adminUserId" -ForegroundColor White
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

Write-Host "👤 Step 2: Target user create kar rahe hain..." -ForegroundColor Cyan
Write-Host ""

# Target user create karo (jisko tokens bhejna hai)
try {
    $targetUserResponse = Invoke-RestMethod -Uri "$baseUrl/signup" `
        -Method POST `
        -ContentType "application/json" `
        -Body '{"email":"user1@example.com","password":"user123"}' `
        -ErrorAction Stop
    
    Write-Host "✅ Target user created successfully!" -ForegroundColor Green
    $targetUserId = $targetUserResponse.user_id
    $targetPublicKey = $targetUserResponse.public_key
    Write-Host "   User ID: $targetUserId" -ForegroundColor White
    Write-Host "   Public Key: $targetPublicKey" -ForegroundColor White
    Write-Host "   Explorer: $($targetUserResponse.solana_explorer_url)" -ForegroundColor Cyan
} catch {
    Write-Host "⚠️  User already exists, using existing user..." -ForegroundColor Yellow
    # Agar already hai, toh user_id 2 assume kar lo (ya database se fetch karo)
    $targetUserId = 2
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

Write-Host "💰 Step 3: Master wallet se user ko 100 KARMM tokens bhej rahe hain..." -ForegroundColor Cyan
Write-Host ""

# Master wallet se user ko tokens send karo
try {
    $rewardResponse = Invoke-RestMethod -Uri "$baseUrl/reward" `
        -Method POST `
        -Headers @{
            "Authorization" = "Bearer $token"
            "Content-Type" = "application/json"
        } `
        -Body "{`"user_id`":$targetUserId,`"tokens`":100.0}" `
        -ErrorAction Stop
    
    Write-Host "✅ Tokens sent successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "   Transaction Details:" -ForegroundColor White
    Write-Host "   ├─ Amount: 100 KARMM tokens" -ForegroundColor White
    Write-Host "   ├─ To User ID: $targetUserId" -ForegroundColor White
    Write-Host "   └─ Transaction Signature: $($rewardResponse.transaction_signature)" -ForegroundColor White
    Write-Host ""
    Write-Host "   🔍 View on Explorer:" -ForegroundColor Cyan
    Write-Host "   https://explorer.solana.com/tx/$($rewardResponse.transaction_signature)" -ForegroundColor Blue
    
} catch {
    Write-Host "❌ Error sending tokens:" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    Write-Host ""
    Write-Host "💡 Common issues:" -ForegroundColor Yellow
    Write-Host "   • Master wallet mein SOL nahi hai? Run: cargo run --bin check_wallet" -ForegroundColor White
    Write-Host "   • Server running hai? Run: cargo run" -ForegroundColor White
    exit 1
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

Write-Host "🎉 Admin Flow Complete!" -ForegroundColor Green
Write-Host ""
Write-Host "📋 Summary:" -ForegroundColor Cyan
Write-Host "   Admin User ID: $adminUserId" -ForegroundColor White
Write-Host "   Target User ID: $targetUserId" -ForegroundColor White
Write-Host "   Tokens Sent: 100 KARMM" -ForegroundColor White
Write-Host ""
Write-Host "💡 Next Steps:" -ForegroundColor Yellow
Write-Host "   • Check user balance: GET $baseUrl/balance (with user's token)" -ForegroundColor White
Write-Host "   • Send more tokens: Use /reward API again" -ForegroundColor White
Write-Host "   • Transfer between users: Use /transfer API" -ForegroundColor White
Write-Host ""
