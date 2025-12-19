//! Example: Placing a position in a time-shifted market

use preda_sdk::PredaClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize client
    let keypair = Keypair::new();
    let client = PredaClient::new("https://api.mainnet-beta.solana.com", keypair).await?;

    println!("🎯 Placing a position in a time-shifted market...\n");

    // Market address (replace with actual market)
    let market_address = Pubkey::from_str("11111111111111111111111111111111")?;

    // Calculate time bucket (predict inflection in 24 hours)
    let current_time = chrono::Utc::now().timestamp();
    let predicted_time = current_time + 86400; // +24 hours

    // Position amount (0.1 SOL)
    let amount = 100_000_000; // lamports

    println!("📍 Position Details:");
    println!("   Market: {}", market_address);
    println!("   Predicted time: {}", predicted_time);
    println!("   Amount: {} lamports (0.1 SOL)", amount);

    // Place position
    let position = client
        .place_position(&market_address, predicted_time, amount)
        .await?;

    println!("\n✅ Position placed successfully!");
    println!("   Position address: {}", position.address);
    println!("   Time bucket: {} - {}", position.time_bucket.start, position.time_bucket.end);
    println!("   Status: {:?}", position.status);
    println!("   Created at: {}", position.created_at);

    // Query current BSI
    println!("\n📊 Current Belief State Index:");
    let bsi = client.get_belief_state_index(&market_address).await?;
    println!("   Value: {:.4}", bsi.value);
    println!("   Velocity: {:.4}", bsi.velocity);
    println!("   Volatility: {:.4}", bsi.volatility);
    println!("   Confidence: {:.2}%", bsi.confidence * 100.0);

    Ok(())
}
