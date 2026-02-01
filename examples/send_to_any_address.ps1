# Send Tokens to Any Solana Address
# Kisi bhi address par tokens bhejne ke liye - database mein registered nahi hona chahiye!

param(
    [string]$ToAddress = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    [double]$Tokens = 5.0
)

$baseUrl = "http://localhost:3000"

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "  🎯 Send Tokens to Any Solana Address" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

# Step 1: Admin login
Write-Host "🔐 Step 1: Admin login..." -ForegroundColor Yellow
Write-Host ""

try {
    $loginResponse = Invoke-RestMethod -Uri "$baseUrl/login" `
        -Method POST `
        -ContentType "application/json" `
        -Body '{"email":"admin@example.com","password":"admin123"}' `
        -ErrorAction Stop
    
    $token = $loginResponse.token
    Write-Host "   ✅ Logged in successfully!" -ForegroundColor Green
    Write-Host "   Admin User ID: $($loginResponse.user_id)" -ForegroundColor White
} catch {
    Write-Host "   ❌ Login failed. Creating admin user..." -ForegroundColor Red
    
    # Try signup if login fails
    try {
        $signupResponse = Invoke-RestMethod -Uri "$baseUrl/signup" `
            -Method POST `
            -ContentType "application/json" `
            -Body '{"email":"admin@example.com","password":"admin123"}' `
            -ErrorAction Stop
        
        $token = $signupResponse.token
        Write-Host "   ✅ Admin user created and logged in!" -ForegroundColor Green
        Write-Host "   Admin User ID: $($signupResponse.user_id)" -ForegroundColor White
    } catch {
        Write-Host "   ❌ Error: $_" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

# Step 2: Send tokens to address
Write-Host "💰 Step 2: Sending $Tokens KARMM tokens..." -ForegroundColor Yellow
Write-Host "   To Address: $ToAddress" -ForegroundColor White
Write-Host ""

try {
    $sendResponse = Invoke-RestMethod -Uri "$baseUrl/send-to-address" `
        -Method POST `
        -Headers @{
            "Authorization" = "Bearer $token"
            "Content-Type" = "application/json"
        } `
        -Body "{`"to_address`":`"$ToAddress`",`"tokens`":$Tokens}" `
        -ErrorAction Stop
    
    Write-Host "   ✅ Tokens sent successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "   Transaction Details:" -ForegroundColor White
    Write-Host "   ├─ Amount: $Tokens KARMM tokens" -ForegroundColor White
    Write-Host "   ├─ To Address: $($sendResponse.to_address)" -ForegroundColor White
    Write-Host "   └─ Transaction: $($sendResponse.transaction_signature)" -ForegroundColor White
    Write-Host ""
    Write-Host "   🔍 View on Solana Explorer:" -ForegroundColor Cyan
    Write-Host "   $($sendResponse.solana_explorer_url)" -ForegroundColor Blue
    Write-Host ""
    
} catch {
    Write-Host "   ❌ Error sending tokens:" -ForegroundColor Red
    $errorDetails = $_.ErrorDetails.Message | ConvertFrom-Json -ErrorAction SilentlyContinue
    if ($errorDetails) {
        Write-Host "   $($errorDetails.error)" -ForegroundColor Red
    } else {
        Write-Host "   $($_.Exception.Message)" -ForegroundColor Red
    }
    Write-Host ""
    Write-Host "   💡 Common issues:" -ForegroundColor Yellow
    Write-Host "   • Master wallet mein SOL kam hai? Run: cargo run --bin check_wallet" -ForegroundColor White
    Write-Host "   • Address valid hai? Check format (base58)" -ForegroundColor White
    Write-Host "   • Server running hai? Run: cargo run" -ForegroundColor White
    exit 1
}

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

Write-Host "🎉 Success!" -ForegroundColor Green
Write-Host ""
Write-Host "📋 Summary:" -ForegroundColor Cyan
Write-Host "   • Tokens Sent: $Tokens KARMM" -ForegroundColor White
Write-Host "   • To Address: $ToAddress" -ForegroundColor White
Write-Host "   • Status: Confirmed on Solana Mainnet" -ForegroundColor White
Write-Host ""
Write-Host "💡 Next Steps:" -ForegroundColor Yellow
Write-Host "   • Recipient ka Phantom wallet khol kar balance check karo" -ForegroundColor White
Write-Host "   • Transaction ko Explorer par verify karo" -ForegroundColor White
Write-Host "   • More addresses par tokens bhejne ke liye yeh script dobara run karo" -ForegroundColor White
Write-Host ""
Write-Host "📝 Usage:" -ForegroundColor Cyan
Write-Host "   .\send_to_any_address.ps1 -ToAddress <address> -Tokens 10.0" -ForegroundColor White
Write-Host ""
