# Phantom Wallet Key Converter
# Converts Phantom wallet private key to hex format for .env file

Write-Host "========================================"
Write-Host "Phantom Wallet Key Converter"
Write-Host "========================================"
Write-Host ""

Write-Host "Phantom wallet se private key export karo:"
Write-Host "1. Phantom extension/app open karo"
Write-Host "2. Settings -> Export Private Key"
Write-Host "3. Password enter karo"
Write-Host "4. Private key copy karo"
Write-Host ""

Write-Host "Key format options:"
Write-Host "A) Hex string (128 characters) - Direct use karo"
Write-Host "B) Base58 string - Convert karna hoga"
Write-Host "C) Array format [123,45,67,...] - Convert karna hoga"
Write-Host ""

$keyFormat = Read-Host "Key format (A/B/C)?"

if ($keyFormat -eq "A" -or $keyFormat -eq "a") {
    Write-Host ""
    Write-Host "Hex string paste karo (128 characters):"
    $hexKey = Read-Host
    
    # Validate
    if ($hexKey.Length -ne 128) {
        Write-Host "⚠️  Error: Key length should be 128 characters. Got: $($hexKey.Length)" -ForegroundColor Red
        exit
    }
    
    # Check hex format
    if ($hexKey -notmatch '^[0-9a-fA-F]{128}$') {
        Write-Host "⚠️  Error: Invalid hex format. Only 0-9, a-f allowed." -ForegroundColor Red
        exit
    }
    
    $finalKey = $hexKey.ToLower()
    
} elseif ($keyFormat -eq "B" -or $keyFormat -eq "b") {
    Write-Host ""
    Write-Host "Base58 string paste karo:"
    Write-Host "Note: Base58 to hex conversion ke liye online tool use karo:"
    Write-Host "https://www.base58check.com/"
    Write-Host ""
    Write-Host "Ya Solana CLI use karo:"
    Write-Host "solana-keygen pubkey <keyfile>"
    exit
    
} elseif ($keyFormat -eq "C" -or $keyFormat -eq "c") {
    Write-Host ""
    Write-Host "Array format paste karo (comma separated numbers):"
    Write-Host "Example: 123,45,67,89,101,113,125,137,149,161,173,185,197,209,221,233,245,1,13,25,37,49,61,73,85,97,109,121,133,145,157,169,181,193,205,217,229,241,253,9,21,33,45,57,69,81,93,105,117,129,141,153,165,177,189,201,213,225,237,249,5,17,29"
    Write-Host ""
    $input = Read-Host "Paste array"
    
    # Parse comma-separated numbers
    try {
        $bytes = $input -split ',' | ForEach-Object { [int]$_.Trim() }
        
        if ($bytes.Count -ne 64) {
            Write-Host "⚠️  Error: Should have 64 numbers. Got: $($bytes.Count)" -ForegroundColor Red
            exit
        }
        
        # Convert to hex
        $finalKey = ($bytes | ForEach-Object { "{0:x2}" -f $_ }) -join ""
        
    } catch {
        Write-Host "⚠️  Error: Invalid array format" -ForegroundColor Red
        exit
    }
    
} else {
    Write-Host "⚠️  Invalid option" -ForegroundColor Red
    exit
}

Write-Host ""
Write-Host "========================================"
Write-Host "✅ Conversion Successful!"
Write-Host "========================================"
Write-Host ""
Write-Host "Master Wallet Private Key (Hex):"
Write-Host $finalKey
Write-Host ""
Write-Host "Length: $($finalKey.Length) characters"
Write-Host ""
Write-Host "========================================"
Write-Host ""
Write-Host "📝 Next Steps:"
Write-Host "1. Copy the hex key above"
Write-Host "2. Open .env file"
Write-Host "3. Find MASTER_WALLET_PRIVATE_KEY"
Write-Host "4. Replace with the key above"
Write-Host ""
Write-Host "⚠️  Keep this SECRET! Never share!"
Write-Host ""

# Save to file
$finalKey | Out-File -FilePath phantom-wallet-key.txt -Encoding utf8 -NoNewline
Write-Host "✅ Saved to: phantom-wallet-key.txt"
Write-Host ""

# Also show public key if possible (for verification)
Write-Host "💡 Tip: Verify wallet address on Solana Explorer:"
Write-Host "https://explorer.solana.com/"
Write-Host ""
