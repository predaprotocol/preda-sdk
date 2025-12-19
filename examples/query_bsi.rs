//! Example: Querying and monitoring Belief State Index

use preda_sdk::{bsi::BeliefMonitor, PredaClient};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::str::FromStr;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize client
    let keypair = Keypair::new();
    let client = PredaClient::new("https://api.mainnet-beta.solana.com", keypair).await?;

    println!("📊 Querying Belief State Index...\n");

    // Market address
    let market_address = Pubkey::from_str("11111111111111111111111111111111")?;

    // Query current BSI
    let bsi = client.get_belief_state_index(&market_address).await?;

    println!("Current BSI:");
    println!("   Domain: {}", bsi.domain);
    println!("   Value: {:.4}", bsi.value);
    println!("   Velocity: {:.4}", bsi.velocity);
    println!("   Volatility: {:.4}", bsi.volatility);
    println!("   Confidence: {:.2}%", bsi.confidence * 100.0);
    println!("   Signal count: {}", bsi.signal_count);
    println!("   Last updated: {}", bsi.last_updated);

    println!("\n🔍 Sentiment Analysis:");
    if bsi.is_bullish() {
        println!("   📈 BULLISH - Positive sentiment detected");
    } else if bsi.is_bearish() {
        println!("   📉 BEARISH - Negative sentiment detected");
    } else {
        println!("   ➡️  NEUTRAL - No clear directional bias");
    }

    if bsi.is_accelerating() {
        println!("   ⚡ ACCELERATING - Rapid belief change in progress");
    }

    if bsi.is_volatile() {
        println!("   ⚠️  HIGH VOLATILITY - Unstable belief state");
    }

    // Set up belief monitor
    println!("\n🔔 Setting up belief monitor for inflection detection...");
    let monitor = BeliefMonitor::new(0.5, 300); // 0.5 threshold, 5 min persistence

    // Register callback
    monitor
        .on_inflection(|inflection| {
            println!("\n🚨 INFLECTION DETECTED!");
            println!("   Type: {:?}", inflection.inflection_type);
            println!("   Timestamp: {}", inflection.timestamp);
            println!("   BSI Value: {:.4}", inflection.bsi_value);
            println!("   Velocity: {:.4}", inflection.velocity);
            println!("   Sharpness: {:.4}", inflection.sharpness);
        })
        .await;

    // Simulate monitoring (in production, this would be a continuous loop)
    println!("   Monitoring for 30 seconds...");
    for i in 0..6 {
        sleep(Duration::from_secs(5)).await;
        let current_bsi = client.get_belief_state_index(&market_address).await?;
        monitor.update(current_bsi.clone()).await?;
        println!("   Update {}: BSI = {:.4}", i + 1, current_bsi.value);
    }

    println!("\n✅ Monitoring complete!");

    Ok(())
}
