#!/bin/bash

echo "==================================================="
echo "     Solana Token Backend - Quick Setup"
echo "==================================================="
echo ""
echo "This script will help you configure the application."
echo ""

# Check if .env already exists
if [ -f .env ]; then
    read -p "[WARNING] A .env file already exists. Do you want to overwrite it? (y/n): " OVERWRITE
    if [[ "$OVERWRITE" != "y" && "$OVERWRITE" != "Y" ]]; then
        echo "Setup cancelled."
        exit 0
    fi
fi

echo ""
echo "--- Step 1: Solana Configuration ---"
read -p "Enter your Solana RPC URL (e.g., Helius/Alchemy link): " RPC_URL
read -p "Enter your Token Mint Address: " MINT_ADDRESS
read -p "Enter your Master Wallet Private Key (Hex format): " MASTER_KEY

echo ""
echo "--- Step 2: Security Configuration ---"
echo "Generating secure keys..."

# Generate random keys
JWT_SECRET=$(openssl rand -base64 32)
ENCRYPTION_KEY=$(openssl rand -hex 32)

echo "JWT Secret generated."
echo "Encryption Key generated."

echo ""
echo "--- Step 3: Saving Configuration ---"

cat > .env <<EOL
# Database Configuration
DATABASE_URL=users.db

# JWT Secret (Auto-generated)
JWT_SECRET=$JWT_SECRET

# Encryption Key (Auto-generated)
ENCRYPTION_KEY=$ENCRYPTION_KEY

# Solana Configuration
SOLANA_RPC_URL=$RPC_URL
KARMM_MINT_ADDRESS=$MINT_ADDRESS
MASTER_WALLET_PRIVATE_KEY=$MASTER_KEY

# Server Configuration
SERVER_PORT=3000
EOL

echo ""
echo "[SUCCESS] Configuration saved to .env file!"
echo ""
echo "==================================================="
echo "Setup Complete!"
echo ""
echo "You can now run the application using:"
echo "   docker-compose up -d"
echo "   OR"
echo "   cargo run"
echo "==================================================="
