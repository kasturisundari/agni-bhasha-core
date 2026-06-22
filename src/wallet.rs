/// KasturiChain Wallet CLI
///
/// Command line interface for generating Dilithium5 keys,
/// checking balances, and sending transactions.

use crate::network::transaction::{generate_keypair, Transaction};
use crate::network::account::ACCOUNT_DB;
use crate::network::daemon::add_to_mempool;

pub async fn handle_wallet_command(args: &[String]) {
    if args.len() < 3 {
        print_wallet_help();
        return;
    }

    match args[2].as_str() {
        "create" => {
            let (pk, sk) = generate_keypair();
            let address = Transaction::address_from_pubkey(&pk);
            
            println!("🕉️  New Kasturi Wallet Created!");
            println!("--------------------------------");
            println!("Address:    {}", address);
            println!("Public Key: {}", hex::encode(&pk));
            println!("Secret Key: {}", hex::encode(&sk));
            println!("--------------------------------");
            println!("⚠️  SAVE YOUR SECRET KEY! It cannot be recovered.");
        }
        "balance" => {
            if args.len() < 4 {
                println!("Usage: kasturisundari wallet balance <address>");
                return;
            }
            let address = &args[3];
            let account_db = ACCOUNT_DB.lock().unwrap();
            let balance = account_db.get_balance(address);
            println!("💰 Balance of {}: {} Bhakti", address, balance);
        }
        "send" => {
            if args.len() < 6 {
                println!("Usage: kasturisundari wallet send <to> <amount> <secret_key_hex>");
                return;
            }
            let to = &args[3];
            let amount: u64 = match args[4].parse() {
                Ok(a) => a,
                Err(_) => {
                    println!("Invalid amount");
                    return;
                }
            };
            let sk_hex = &args[5];
            
            let sk_bytes = match hex::decode(sk_hex) {
                Ok(b) => b,
                Err(_) => {
                    println!("Invalid secret key format");
                    return;
                }
            };

            // To sign we need both sk and pk. But Dilithium5 SecretKey structure actually contains the public key too.
            // Let's derive the public key from the secret key (for simplicity we will use pqcrypto_dilithium to load it).
            use pqcrypto_dilithium::dilithium5::*;
            use pqcrypto_traits::sign::SecretKey;
            
            let sk = match pqcrypto_dilithium::dilithium5::SecretKey::from_bytes(&sk_bytes) {
                Ok(k) => k,
                Err(_) => {
                    println!("Failed to parse secret key");
                    return;
                }
            };

            // In CRYSTALS-Dilithium, the public key is the last 2592 bytes of the 4864-byte secret key (approximately).
            // Actually, `pqcrypto` doesn't expose a `to_public_key` method directly on SecretKey.
            // For the CLI, we will just ask the user for both if they want to sign, or we can use the `public_key` field in the secret key struct.
            // But pqcrypto doesn't expose it. So we need the public key passed in.
            // Let's modify the usage: `send <to> <amount> <public_key_hex> <secret_key_hex>`
            if args.len() < 7 {
                println!("Usage: kasturisundari wallet send <to> <amount> <public_key_hex> <secret_key_hex>");
                return;
            }
            let pk_hex = &args[5];
            let sk_hex = &args[6];

            let pk_bytes = match hex::decode(pk_hex) {
                Ok(b) => b,
                Err(_) => {
                    println!("Invalid public key format");
                    return;
                }
            };
            let sk_bytes = match hex::decode(sk_hex) {
                Ok(b) => b,
                Err(_) => {
                    println!("Invalid secret key format");
                    return;
                }
            };

            let from_address = Transaction::address_from_pubkey(&pk_bytes);
            let nonce = {
                let db = ACCOUNT_DB.lock().unwrap();
                db.get_nonce(&from_address) + 1
            };

            let mut tx = Transaction::new(
                String::new(), // will be filled by sign
                to.to_string(),
                amount,
                nonce,
                "CLI Transfer".into()
            );

            if let Err(e) = tx.sign(&sk_bytes, &pk_bytes) {
                println!("Failed to sign transaction: {}", e);
                return;
            }

            println!("✅ Transaction signed successfully.");
            println!("   Tx Hash: {}", tx.hash());
            
            let tx_json = serde_json::to_value(&tx).unwrap();
            
            // Add to mempool
            add_to_mempool(tx_json.clone());
            
            // Broadcast
            crate::network::gossip::broadcast_transaction(tx_json).await;
            
            println!("📡 Transaction broadcasted to network.");
        }
        _ => print_wallet_help(),
    }
}

fn print_wallet_help() {
    println!("KasturiChain Wallet CLI");
    println!("Commands:");
    println!("  create                                      - Generate a new wallet");
    println!("  balance <address>                           - Check account balance");
    println!("  send <to> <amount> <pub_key> <sec_key>      - Send Bhakti");
}
