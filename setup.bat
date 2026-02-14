@echo off
setlocal EnableDelayedExpansion

echo ===================================================
echo      Solana Token Backend - Quick Setup
echo ===================================================
echo.
echo This script will help you configure the application.
echo.

:: Check if .env already exists
if exist .env (
    echo [WARNING] A .env file already exists.
    set /p "OVERWRITE=Do you want to overwrite it? (Y/N): "
    if /i "!OVERWRITE!" neq "Y" (
        echo Setup cancelled.
        goto :EOF
    )
)

echo.
echo --- Step 1: Solana Configuration ---
set /p "RPC_URL=Enter your Solana RPC URL (e.g., Helius/Alchemy link): "
set /p "MINT_ADDRESS=Enter your Token Mint Address: "
set /p "MASTER_KEY=Enter your Master Wallet Private Key (Hex format): "

echo.
echo --- Step 2: Security Configuration ---
echo Generating secure keys...

:: Generate a pseudo-random JWT Secret (using PowerShell for better randomness)
for /f "delims=" %%i in ('powershell -Command "[Convert]::ToBase64String((1..32 | ForEach-Object { Get-Random -Minimum 0 -Maximum 255 } | ForEach-Object { [byte]$_ }))"') do set JWT_SECRET=%%i

:: Generate a pseudo-random Encryption Key (64 hex chars)
for /f "delims=" %%i in ('powershell -Command -NoProfile -ExecutionPolicy Bypass "Write-Output -NoEnumerate ((1..32 | ForEach-Object { '{0:x2}' -f (Get-Random -Minimum 0 -Maximum 256) }) -join '')"') do set ENCRYPTION_KEY=%%i

echo.
echo JWT Secret generated.
echo Encryption Key generated.

echo.
echo --- Step 3: Saving Configuration ---

(
echo # Database Configuration
echo DATABASE_URL=users.db
echo.
echo # JWT Secret ^(Auto-generated^)
echo JWT_SECRET=!JWT_SECRET!
echo.
echo # Encryption Key ^(Auto-generated^)
echo ENCRYPTION_KEY=!ENCRYPTION_KEY!
echo.
echo # Solana Configuration
echo SOLANA_RPC_URL=!RPC_URL!
echo KARMM_MINT_ADDRESS=!MINT_ADDRESS!
echo MASTER_WALLET_PRIVATE_KEY=!MASTER_KEY!
echo.
echo # Server Configuration
echo SERVER_PORT=3000
) > .env

echo.
echo [SUCCESS] Configuration saved to .env file!
echo.
echo ===================================================
echo Setup Complete!
echo.
echo You can now run the application using:
echo    docker-compose up -d
echo    OR
echo    cargo run
echo ===================================================

pause
