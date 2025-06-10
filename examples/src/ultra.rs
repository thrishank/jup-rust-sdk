use jup::sign_transaction;
use jup_ag_sdk::{
    JupiterClient,
    types::{UltraExecuteOrderRequest, UltraOrderRequest},
};

pub async fn ultra() {
    // Initialize the client
    let client = JupiterClient::new("https://lite-api.jup.ag");

    // Create an ultra order request to swap 10 USDC to SOL
    let ultra_req = UltraOrderRequest::new(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "So11111111111111111111111111111111111111112",
        10_000_000,
    )
    .add_taker("your taker wallet address");

    // Fetch ultra order
    let ultra_res = client
        .get_ultra_order(&ultra_req)
        .await
        .expect("Failed to get ultra order");

    let base64_signed_tx = sign_transaction(
        ultra_res
            .transaction
            .expect("no transaction found in ultra response"),
    );

    // Create execute order request
    let execute = UltraExecuteOrderRequest {
        signed_transaction: base64_signed_tx,
        request_id: ultra_res.request_id,
    };

    // Execute the transaction
    let execute_res = client
        .ultra_execute_order(&execute)
        .await
        .expect("Failed to execute transaction");

    println!(
        "Transaction: {}",
        execute_res
            .signature
            .expect("signature not found in execute response")
    );
}
