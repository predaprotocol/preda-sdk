//! Belief-related type definitions

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// Belief State Index - core measurement construct of Preda
///
/// Represents a continuously updated aggregation of belief signals across defined domains.
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct BeliefStateIndex {
    /// Current BSI value (normalized -1.0 to 1.0)
    pub value: f64,

    /// Rate of change (velocity)
    pub velocity: f64,

    /// Volatility measure
    pub volatility: f64,

    /// Timestamp of last update (Unix timestamp)
    pub last_updated: i64,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,

    /// Number of signals aggregated
    pub signal_count: u32,

    /// Domain identifier
    pub domain: String,
}

/// Belief condition types for market resolution
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum BeliefCondition {
    /// Sentiment polarity shift
    SentimentShift {
        from_polarity: f64,
        to_polarity: f64,
        persistence_window: u64, // seconds
    },

    /// Probability threshold crossing
    ProbabilityThreshold {
        threshold: f64,
        direction: ThresholdDirection,
        persistence_window: u64,
    },

    /// Model consensus convergence
    ModelConsensus {
        min_models: u32,
        convergence_band: f64,
        persistence_window: u64,
    },

    /// Narrative velocity threshold
    NarrativeVelocity {
        velocity_threshold: f64,
        acceleration_threshold: f64,
        persistence_window: u64,
    },

    /// Custom belief condition
    Custom {
        condition_type: String,
        parameters: Vec<(String, f64)>,
        persistence_window: u64,
    },
}

/// Threshold direction for probability-based conditions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum ThresholdDirection {
    Above,
    Below,
    Cross,
}

/// Belief inflection point detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefInflection {
    /// Type of inflection detected
    pub inflection_type: InflectionType,

    /// Timestamp when inflection occurred
    pub timestamp: i64,

    /// BSI value at inflection
    pub bsi_value: f64,

    /// Velocity at inflection
    pub velocity: f64,

    /// Sharpness of inflection (0.0 to 1.0)
    pub sharpness: f64,

    /// Persistence duration (seconds)
    pub persistence_duration: u64,

    /// Validation status
    pub validated: bool,
}

/// Types of belief inflections
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InflectionType {
    SentimentReversal,
    ThresholdCrossing,
    ConsensusFormation,
    ConsensusFragmentation,
    VelocitySpike,
    VelocityStabilization,
}

/// Individual belief signal from an oracle
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct BeliefSignal {
    /// Signal source identifier
    pub source: String,

    /// Signal type
    pub signal_type: SignalType,

    /// Signal value (normalized)
    pub value: f64,

    /// Signal weight in aggregation
    pub weight: f64,

    /// Timestamp of signal
    pub timestamp: i64,

    /// Signal metadata
    pub metadata: Vec<(String, String)>,
}

/// Types of belief signals
#[derive(Debug, Clone, Copy, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum SignalType {
    Sentiment,
    Probability,
    Narrative,
    ModelForecast,
    ConsensusMetric,
}

impl BeliefStateIndex {
    /// Create a new BSI with default values
    pub fn new(domain: String) -> Self {
        Self {
            value: 0.0,
            velocity: 0.0,
            volatility: 0.0,
            last_updated: 0,
            confidence: 0.0,
            signal_count: 0,
            domain,
        }
    }

    /// Check if BSI indicates bullish sentiment
    pub fn is_bullish(&self) -> bool {
        self.value > 0.3
    }

    /// Check if BSI indicates bearish sentiment
    pub fn is_bearish(&self) -> bool {
        self.value < -0.3
    }

    /// Check if BSI is neutral
    pub fn is_neutral(&self) -> bool {
        self.value.abs() <= 0.3
    }

    /// Check if BSI is accelerating
    pub fn is_accelerating(&self) -> bool {
        self.velocity.abs() > 0.1
    }

    /// Check if BSI is volatile
    pub fn is_volatile(&self) -> bool {
        self.volatility > 0.5
    }
}

impl BeliefCondition {
    /// Get the persistence window for this condition
    pub fn persistence_window(&self) -> u64 {
        match self {
            BeliefCondition::SentimentShift { persistence_window, .. } => *persistence_window,
            BeliefCondition::ProbabilityThreshold { persistence_window, .. } => *persistence_window,
            BeliefCondition::ModelConsensus { persistence_window, .. } => *persistence_window,
            BeliefCondition::NarrativeVelocity { persistence_window, .. } => *persistence_window,
            BeliefCondition::Custom { persistence_window, .. } => *persistence_window,
        }
    }

    /// Validate the belief condition parameters
    pub fn validate(&self) -> Result<(), String> {
        match self {
            BeliefCondition::SentimentShift { from_polarity, to_polarity, .. } => {
                if from_polarity.abs() > 1.0 || to_polarity.abs() > 1.0 {
                    return Err("Polarity values must be between -1.0 and 1.0".to_string());
                }
                Ok(())
            }
            BeliefCondition::ProbabilityThreshold { threshold, .. } => {
                if *threshold < 0.0 || *threshold > 1.0 {
                    return Err("Threshold must be between 0.0 and 1.0".to_string());
                }
                Ok(())
            }
            BeliefCondition::ModelConsensus { min_models, convergence_band, .. } => {
                if *min_models < 2 {
                    return Err("Minimum 2 models required for consensus".to_string());
                }
                if *convergence_band <= 0.0 || *convergence_band > 1.0 {
                    return Err("Convergence band must be between 0.0 and 1.0".to_string());
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsi_sentiment_detection() {
        let mut bsi = BeliefStateIndex::new("BTC".to_string());
        
        bsi.value = 0.5;
        assert!(bsi.is_bullish());
        assert!(!bsi.is_bearish());
        
        bsi.value = -0.5;
        assert!(bsi.is_bearish());
        assert!(!bsi.is_bullish());
        
        bsi.value = 0.1;
        assert!(bsi.is_neutral());
    }

    #[test]
    fn test_belief_condition_validation() {
        let valid_condition = BeliefCondition::SentimentShift {
            from_polarity: -0.2,
            to_polarity: 0.6,
            persistence_window: 3600,
        };
        assert!(valid_condition.validate().is_ok());

        let invalid_condition = BeliefCondition::SentimentShift {
            from_polarity: -1.5,
            to_polarity: 0.6,
            persistence_window: 3600,
        };
        assert!(invalid_condition.validate().is_err());
    }
}
