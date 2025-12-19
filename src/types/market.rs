//! Market-related type definitions

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use super::belief::BeliefCondition;

/// Market structure for time-shifted prediction markets
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Market {
    /// Market address on Solana
    pub address: Pubkey,

    /// Market creator
    pub creator: Pubkey,

    /// Market type
    pub market_type: MarketType,

    /// Belief condition for resolution
    pub belief_condition: BeliefCondition,

    /// Market description
    pub description: String,

    /// Current market state
    pub state: MarketState,

    /// Market configuration
    pub config: MarketConfig,

    /// Creation timestamp
    pub created_at: i64,

    /// Resolution timestamp (if resolved)
    pub resolved_at: Option<i64>,

    /// Total value locked in market
    pub total_value_locked: u64,

    /// Number of participants
    pub participant_count: u32,

    /// Oracle addresses
    pub oracle_addresses: Vec<Pubkey>,
}

/// Market types supported by Preda
#[derive(Debug, Clone, Copy, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum MarketType {
    /// Sentiment transition markets
    SentimentTransition,

    /// Probability threshold markets
    ProbabilityThreshold,

    /// Model consensus markets
    ModelConsensus,

    /// Narrative velocity markets
    NarrativeVelocity,
}

/// Market lifecycle states
#[derive(Debug, Clone, Copy, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum MarketState {
    /// Market is being initialized
    Initializing,

    /// Market is active and accepting positions
    Active,

    /// Market is monitoring for belief inflection
    Monitoring,

    /// Belief condition met, pending validation
    InflectionDetected,

    /// Market resolved
    Resolved,

    /// Market cancelled
    Cancelled,

    /// Market expired without resolution
    Expired,
}

/// Market configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct MarketConfig {
    /// Time bucket granularity (seconds)
    pub time_bucket_size: u64,

    /// Minimum position size (lamports)
    pub min_position_size: u64,

    /// Maximum position size (lamports)
    pub max_position_size: u64,

    /// Market expiration timestamp
    pub expiration_time: i64,

    /// Oracle update frequency (seconds)
    pub oracle_update_frequency: u64,

    /// Volatility adjustment factor
    pub volatility_factor: f64,

    /// Settlement curve type
    pub settlement_curve: SettlementCurve,

    /// Fee percentage (basis points)
    pub fee_bps: u16,
}

/// Settlement curve types for volatility-aware payouts
#[derive(Debug, Clone, Copy, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum SettlementCurve {
    /// Linear payout based on timing accuracy
    Linear,

    /// Exponential decay from inflection point
    Exponential,

    /// Gaussian distribution around inflection
    Gaussian,

    /// Custom curve with parameters
    Custom,
}

impl Market {
    /// Check if market is active
    pub fn is_active(&self) -> bool {
        matches!(self.state, MarketState::Active | MarketState::Monitoring)
    }

    /// Check if market is resolved
    pub fn is_resolved(&self) -> bool {
        matches!(self.state, MarketState::Resolved)
    }

    /// Check if market has expired
    pub fn has_expired(&self, current_time: i64) -> bool {
        current_time > self.config.expiration_time
    }

    /// Check if market can accept new positions
    pub fn can_accept_positions(&self) -> bool {
        self.state == MarketState::Active
    }

    /// Get time until expiration (seconds)
    pub fn time_until_expiration(&self, current_time: i64) -> i64 {
        self.config.expiration_time - current_time
    }
}

impl MarketConfig {
    /// Create default market configuration
    pub fn default() -> Self {
        Self {
            time_bucket_size: 3600,        // 1 hour
            min_position_size: 1_000_000,  // 0.001 SOL
            max_position_size: 1_000_000_000, // 1 SOL
            expiration_time: 0,
            oracle_update_frequency: 300,   // 5 minutes
            volatility_factor: 1.0,
            settlement_curve: SettlementCurve::Gaussian,
            fee_bps: 50, // 0.5%
        }
    }

    /// Validate market configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.time_bucket_size == 0 {
            return Err("Time bucket size must be greater than 0".to_string());
        }

        if self.min_position_size > self.max_position_size {
            return Err("Min position size cannot exceed max position size".to_string());
        }

        if self.oracle_update_frequency == 0 {
            return Err("Oracle update frequency must be greater than 0".to_string());
        }

        if self.volatility_factor <= 0.0 {
            return Err("Volatility factor must be positive".to_string());
        }

        if self.fee_bps > 10000 {
            return Err("Fee cannot exceed 100%".to_string());
        }

        Ok(())
    }

    /// Calculate fee amount for a given position size
    pub fn calculate_fee(&self, position_size: u64) -> u64 {
        (position_size as u128 * self.fee_bps as u128 / 10000) as u64
    }
}

impl MarketType {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            MarketType::SentimentTransition => "Sentiment Transition",
            MarketType::ProbabilityThreshold => "Probability Threshold",
            MarketType::ModelConsensus => "Model Consensus",
            MarketType::NarrativeVelocity => "Narrative Velocity",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            MarketType::SentimentTransition => "Resolves when sentiment crosses sustained threshold",
            MarketType::ProbabilityThreshold => "Resolves when probability exceeds defined level",
            MarketType::ModelConsensus => "Resolves when models converge on shared assessment",
            MarketType::NarrativeVelocity => "Resolves when belief change accelerates",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_config_validation() {
        let mut config = MarketConfig::default();
        assert!(config.validate().is_ok());

        config.min_position_size = 2_000_000_000;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_fee_calculation() {
        let config = MarketConfig::default();
        let fee = config.calculate_fee(1_000_000_000); // 1 SOL
        assert_eq!(fee, 5_000_000); // 0.5% = 0.005 SOL
    }

    #[test]
    fn test_market_state_checks() {
        let market = Market {
            address: Pubkey::new_unique(),
            creator: Pubkey::new_unique(),
            market_type: MarketType::SentimentTransition,
            belief_condition: super::super::belief::BeliefCondition::SentimentShift {
                from_polarity: -0.2,
                to_polarity: 0.6,
                persistence_window: 3600,
            },
            description: "Test market".to_string(),
            state: MarketState::Active,
            config: MarketConfig::default(),
            created_at: 0,
            resolved_at: None,
            total_value_locked: 0,
            participant_count: 0,
            oracle_addresses: vec![],
        };

        assert!(market.is_active());
        assert!(!market.is_resolved());
        assert!(market.can_accept_positions());
    }
}
