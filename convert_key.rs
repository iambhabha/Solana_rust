// Quick utility to convert Phantom's base58 private key to hex format
// Usage: cargo run --bin convert_key

use bs58;
use hex;

fn main() {
    // Your Phantom wallet exported private key (base58)
    let base58_key = "2Ku88DuRemBYgEdAuJioNRazbPtZXF4er3qCLGEoTEMypVkAquMFRcx7co5ZSzU58k1tR9UijNQPvNFLYfeQEZpH";
    
    println!("🔑 Converting Phantom Private Key to Hex Format\n");
    println!("Input (base58): {}", base58_key);
    println!();
    
    // Decode from base58
    match bs58::decode(base58_key).into_vec() {
        Ok(bytes) => {
            println!("✅ Decoded successfully!");
            println!("   Length: {} bytes", bytes.len());
            
            // Convert to hex
            let hex_key = hex::encode(&bytes);
            println!();
            println!("🎯 HEX FORMAT (use this in .env):");
            println!("   {}", hex_key);
            println!();
            println!("📝 Copy this to your .env file:");
            println!("   MASTER_WALLET_PRIVATE_KEY={}", hex_key);
        }
        Err(e) => {
            println!("❌ Error decoding: {}", e);
        }
    }
}
