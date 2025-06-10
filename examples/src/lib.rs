use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use bincode::{deserialize, serialize};
use dotenv::dotenv;

use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};
use std::env;

pub fn sign_transaction(transaction: String) -> String {
    // Load .env variables into std::env
    dotenv().ok();

    // Read the variable
    let key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env");

    // if you have the private key in base58 format. Else skip this decode
    let key_bytes = bs58::decode(key)
        .into_vec()
        .expect("Failed to decode base58 private key");

    let keypair = Keypair::from_bytes(&key_bytes).expect("Failed to create Keypair");

    // decode the base64 transaction
    let swap_tx_bytes = STANDARD
        .decode(transaction)
        .expect("Failed to decode base64 transaction");

    // deserialize the transaction bytes into a VersionedTransaction
    let mut tx: VersionedTransaction = deserialize(&swap_tx_bytes).unwrap();
    let message = tx.message.serialize();

    // sign the transaction with the keypair
    let signature = keypair.sign_message(&message);

    // If there is a need for multiple signatures (fee payer), you can modify this part accordingly.

    if tx.signatures.is_empty() {
        tx.signatures.push(signature);
    } else {
        tx.signatures[0] = signature;
    };

    let signed_tx_bytes = serialize(&tx).unwrap();
    STANDARD.encode(&signed_tx_bytes)
}
