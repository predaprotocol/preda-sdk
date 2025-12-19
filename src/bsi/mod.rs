//! Belief State Index (BSI) module
//!
//! The BSI is the core measurement construct of Preda, representing a continuously
//! updated aggregation of belief signals across defined domains.

pub mod aggregator;
pub mod calculator;
pub mod monitor;

pub use crate::types::belief::BeliefStateIndex;
pub use aggregator::SignalAggregator;
pub use calculator::BsiCalculator;
pub use monitor::BeliefMonitor;

use crate::types::belief::BeliefSignal;

/// BSI update event
#[derive(Debug, Clone)]
pub struct BsiUpdate {
    /// Updated BSI
    pub bsi: BeliefStateIndex,

    /// Signals that contributed to this update
    pub signals: Vec<BeliefSignal>,

    /// Update timestamp
    pub timestamp: i64,
}

/// BSI configuration
#[derive(Debug, Clone)]
pub struct BsiConfig {
    /// Temporal smoothing window (seconds)
    pub smoothing_window: u64,

    /// Time decay factor (0.0 to 1.0)
    pub decay_factor: f64,

    /// Minimum signal count for confidence
    pub min_signal_count: u32,

    /// Outlier detection threshold (standard deviations)
    pub outlier_threshold: f64,

    /// Signal weights by type
    pub signal_weights: SignalWeights,
}

/// Signal weights for different oracle types
#[derive(Debug, Clone)]
pub struct SignalWeights {
    pub sentiment: f64,
    pub probability: f64,
    pub narrative: f64,
    pub model_forecast: f64,
    pub consensus_metric: f64,
}

impl Default for BsiConfig {
    fn default() -> Self {
        Self {
            smoothing_window: 300,  // 5 minutes
            decay_factor: 0.95,
            min_signal_count: 3,
            outlier_threshold: 2.5,
            signal_weights: SignalWeights::default(),
        }
    }
}

impl Default for SignalWeights {
    fn default() -> Self {
        Self {
            sentiment: 1.0,
            probability: 1.2,
            narrative: 0.8,
            model_forecast: 1.5,
            consensus_metric: 1.3,
        }
    }
}

impl BsiConfig {
    /// Validate BSI configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.smoothing_window == 0 {
            return Err("Smoothing window must be greater than 0".to_string());
        }

        if self.decay_factor <= 0.0 || self.decay_factor > 1.0 {
            return Err("Decay factor must be between 0.0 and 1.0".to_string());
        }

        if self.min_signal_count == 0 {
            return Err("Minimum signal count must be greater than 0".to_string());
        }

        if self.outlier_threshold <= 0.0 {
            return Err("Outlier threshold must be positive".to_string());
        }

        Ok(())
    }
}
