//! Example: Creating a time-shifted prediction market

use preda_sdk::{BeliefCondition, MarketType, PredaClient};
use solana_sdk::signature::Keypair;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize client
    let keypair = Keypair::new();
    let client = PredaClient::new("https://api.mainnet-beta.solana.com", keypair).await?;

    println!("🚀 Creating a sentiment transition market...\n");

    // Define belief condition
    let belief_condition = BeliefCondition::SentimentShift {
        from_polarity: -0.2,
        to_polarity: 0.6,
        persistence_window: 3600, // 1 hour
    };

    // Create market
    let market = client
        .create_market(
            MarketType::SentimentTransition,
            belief_condition,
            "BTC sentiment turns bullish - predicting when collective belief shifts positive",
        )
        .await?;

    println!("✅ Market created successfully!");
    println!("   Address: {}", market.address);
    println!("   Type: {}", market.market_type.name());
    println!("   Description: {}", market.description);
    println!("   State: {:?}", market.state);
    println!("   Created at: {}", market.created_at);
    println!("\n📊 Market Configuration:");
    println!("   Time bucket size: {} seconds", market.config.time_bucket_size);
    println!("   Min position: {} lamports", market.config.min_position_size);
    println!("   Max position: {} lamports", market.config.max_position_size);
    println!("   Fee: {}%", market.config.fee_bps as f64 / 100.0);

    Ok(())
}
