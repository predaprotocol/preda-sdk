//! # Preda SDK
//!
//! Rust SDK for Time-Shifted Prediction Markets on Solana.
//!
//! ## Overview
//!
//! Preda is a decentralized forecasting system that predicts **when collective belief will change**
//! rather than what will happen. This SDK provides a comprehensive interface for:
//!
//! - Creating and managing time-shifted prediction markets
//! - Querying and monitoring Belief State Index (BSI)
//! - Integrating with multiple oracle types
//! - Placing and managing positions
//! - Analyzing belief dynamics and inflection points
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use preda_sdk::{PredaClient, MarketType, BeliefCondition};
//! use solana_sdk::signature::Keypair;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let keypair = Keypair::new();
//!     let client = PredaClient::new(
//!         "https://api.mainnet-beta.solana.com",
//!         keypair,
//!     ).await?;
//!
//!     let market = client.create_market(
//!         MarketType::SentimentTransition,
//!         BeliefCondition::SentimentShift {
//!             from_polarity: -0.2,
//!             to_polarity: 0.6,
//!             persistence_window: 3600,
//!         },
//!         "BTC sentiment turns bullish",
//!     ).await?;
//!
//!     println!("Market created: {}", market.address);
//!     Ok(())
//! }
//! ```

pub mod bsi;
pub mod client;
pub mod error;
pub mod market;
pub mod oracle;
pub mod types;

// Re-export commonly used types
pub use client::PredaClient;
pub use error::{PredaError, Result};
pub use types::{
    belief::{BeliefCondition, BeliefInflection, BeliefSignal},
    market::{Market, MarketState, MarketType},
    position::{Position, TimeBucket},
};

/// SDK version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default Solana cluster endpoint
pub const DEFAULT_CLUSTER: &str = "https://api.mainnet-beta.solana.com";

/// Preda program ID on Solana
pub const PREDA_PROGRAM_ID: &str = "PredaXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_constants() {
        assert!(!DEFAULT_CLUSTER.is_empty());
        assert!(!PREDA_PROGRAM_ID.is_empty());
    }
}
