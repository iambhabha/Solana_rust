# Transfer KARMM Tokens to Any Solana Address
# Kisi bhi address par tokens bhejne ke liye

param(
    [Parameter(Mandatory=$true)]
    [string]$ToAddress,
    
    [Parameter(Mandatory=$true)]
    [double]$Tokens,
    
    [string]$Email = "user@example.com",
    [string]$Password = "user123"
)

$baseUrl = "http://localhost:3000"

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "  💸 Transfer KARMM Tokens to Address" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

# Step 1: Login
Write-Host "🔐 Step 1: Logging in..." -ForegroundColor Yellow
Write-Host ""

try {
    $loginResponse = Invoke-RestMethod -Uri "$baseUrl/login" `
        -Method POST `
        -ContentType "application/json" `
        -Body "{`"email`":`"$Email`",`"password`":`"$Password`"}" `
        -ErrorAction Stop
    
    $token = $loginResponse.token
    $userId = $loginResponse.user_id
    
    Write-Host "   ✅ Logged in successfully!" -ForegroundColor Green
    Write-Host "   User ID: $userId" -ForegroundColor White
    Write-Host "   Email: $Email" -ForegroundColor White
    
} catch {
    Write-Host "   ❌ Login failed!" -ForegroundColor Red
    Write-Host "   Error: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host ""
    Write-Host "   💡 Make sure:" -ForegroundColor Yellow
    Write-Host "   • User is registered (run signup first)" -ForegroundColor White
    Write-Host "   • Email and password are correct" -ForegroundColor White
    Write-Host "   • Server is running (cargo run)" -ForegroundColor White
    exit 1
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

# Step 2: Check balance
Write-Host "💰 Step 2: Checking your balance..." -ForegroundColor Yellow
Write-Host ""

try {
    $balanceResponse = Invoke-RestMethod -Uri "$baseUrl/balance" `
        -Method GET `
        -Headers @{"Authorization" = "Bearer $token"} `
        -ErrorAction Stop
    
    $currentBalance = $balanceResponse.balance / 1000000000
    
    Write-Host "   Current Balance: $currentBalance KARMM" -ForegroundColor White
    
    if ($currentBalance -lt $Tokens) {
        Write-Host "   ❌ Insufficient balance!" -ForegroundColor Red
        Write-Host "   Need: $Tokens KARMM" -ForegroundColor Red
        Write-Host "   Have: $currentBalance KARMM" -ForegroundColor Red
        Write-Host ""
        Write-Host "   💡 Get tokens first:" -ForegroundColor Yellow
        Write-Host "   • Ask admin to send you tokens (/reward API)" -ForegroundColor White
        Write-Host "   • Or buy tokens (/buy-token API)" -ForegroundColor White
        exit 1
    } else {
        Write-Host "   ✅ Sufficient balance" -ForegroundColor Green
    }
    
} catch {
    Write-Host "   ⚠️  Could not check balance" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

# Step 3: Transfer tokens
Write-Host "📤 Step 3: Transferring $Tokens KARMM tokens..." -ForegroundColor Yellow
Write-Host "   To Address: $ToAddress" -ForegroundColor White
Write-Host ""

try {
    $transferResponse = Invoke-RestMethod -Uri "$baseUrl/transfer" `
        -Method POST `
        -Headers @{
            "Authorization" = "Bearer $token"
            "Content-Type" = "application/json"
        } `
        -Body "{`"to`":`"$ToAddress`",`"tokens`":$Tokens}" `
        -ErrorAction Stop
    
    Write-Host "   ✅ Transfer successful!" -ForegroundColor Green
    Write-Host ""
    Write-Host "   Transaction Details:" -ForegroundColor White
    Write-Host "   ├─ From: Your wallet" -ForegroundColor White
    Write-Host "   ├─ To: $ToAddress" -ForegroundColor White
    Write-Host "   ├─ Amount: $Tokens KARMM tokens" -ForegroundColor White
    Write-Host "   └─ Transaction ID: $($transferResponse.transaction_signature)" -ForegroundColor White
    Write-Host ""
    Write-Host "   🔍 View on Solana Explorer:" -ForegroundColor Cyan
    Write-Host "   https://explorer.solana.com/tx/$($transferResponse.transaction_signature)" -ForegroundColor Blue
    Write-Host ""
    
} catch {
    Write-Host "   ❌ Transfer failed!" -ForegroundColor Red
    
    $errorDetails = $_.ErrorDetails.Message | ConvertFrom-Json -ErrorAction SilentlyContinue
    if ($errorDetails) {
        Write-Host "   Error: $($errorDetails.error)" -ForegroundColor Red
    } else {
        Write-Host "   Error: $($_.Exception.Message)" -ForegroundColor Red
    }
    
    Write-Host ""
    Write-Host "   💡 Common issues:" -ForegroundColor Yellow
    Write-Host "   • Insufficient balance" -ForegroundColor White
    Write-Host "   • Invalid address format" -ForegroundColor White
    Write-Host "   • Master wallet needs SOL for fees (check: cargo run --bin check_wallet)" -ForegroundColor White
    exit 1
}

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

Write-Host "🎉 Success!" -ForegroundColor Green
Write-Host ""
Write-Host "📋 Summary:" -ForegroundColor Cyan
Write-Host "   • Transferred: $Tokens KARMM" -ForegroundColor White
Write-Host "   • To: $ToAddress" -ForegroundColor White
Write-Host "   • Status: Confirmed on Solana Mainnet" -ForegroundColor White
Write-Host ""
Write-Host "💡 Next Steps:" -ForegroundColor Yellow
Write-Host "   • Recipient ka Phantom wallet check karo" -ForegroundColor White
Write-Host "   • Transaction Explorer par verify karo" -ForegroundColor White
Write-Host "   • Your new balance check karo: GET /balance" -ForegroundColor White
Write-Host ""
Write-Host "📝 Usage:" -ForegroundColor Cyan
Write-Host "   .\transfer_to_address.ps1 -ToAddress <address> -Tokens 5.0" -ForegroundColor White
Write-Host "   .\transfer_to_address.ps1 -ToAddress <address> -Tokens 10.0 -Email admin@test.com -Password admin123" -ForegroundColor White
Write-Host ""
