//! BSI Calculator - Core logic for computing Belief State Index

use crate::types::belief::{BeliefSignal, BeliefStateIndex, SignalType};
use crate::bsi::{BsiConfig, SignalWeights};

/// Calculator for Belief State Index
pub struct BsiCalculator {
    config: BsiConfig,
    history: Vec<BeliefStateIndex>,
}

impl BsiCalculator {
    /// Create a new BSI calculator
    pub fn new(config: BsiConfig) -> Self {
        Self {
            config,
            history: Vec::new(),
        }
    }

    /// Calculate BSI from a set of belief signals
    pub fn calculate(&mut self, signals: &[BeliefSignal], domain: String) -> BeliefStateIndex {
        // Filter outliers
        let filtered_signals = self.filter_outliers(signals);

        // Apply weights
        let weighted_signals = self.apply_weights(&filtered_signals);

        // Calculate base value
        let value = self.calculate_weighted_average(&weighted_signals);

        // Calculate velocity
        let velocity = self.calculate_velocity(value);

        // Calculate volatility
        let volatility = self.calculate_volatility(&filtered_signals);

        // Calculate confidence
        let confidence = self.calculate_confidence(&filtered_signals);

        let bsi = BeliefStateIndex {
            value,
            velocity,
            volatility,
            last_updated: chrono::Utc::now().timestamp(),
            confidence,
            signal_count: filtered_signals.len() as u32,
            domain,
        };

        // Store in history
        self.history.push(bsi.clone());
        
        // Keep only recent history
        if self.history.len() > 1000 {
            self.history.remove(0);
        }

        bsi
    }

    /// Filter outlier signals using z-score
    fn filter_outliers(&self, signals: &[BeliefSignal]) -> Vec<BeliefSignal> {
        if signals.len() < 3 {
            return signals.to_vec();
        }

        // Calculate mean and standard deviation
        let mean = signals.iter().map(|s| s.value).sum::<f64>() / signals.len() as f64;
        let variance = signals
            .iter()
            .map(|s| (s.value - mean).powi(2))
            .sum::<f64>()
            / signals.len() as f64;
        let std_dev = variance.sqrt();

        // Filter signals within threshold
        signals
            .iter()
            .filter(|s| {
                let z_score = ((s.value - mean) / std_dev).abs();
                z_score <= self.config.outlier_threshold
            })
            .cloned()
            .collect()
    }

    /// Apply signal type weights
    fn apply_weights(&self, signals: &[BeliefSignal]) -> Vec<(f64, f64)> {
        signals
            .iter()
            .map(|signal| {
                let type_weight = self.get_signal_type_weight(signal.signal_type);
                let total_weight = signal.weight * type_weight;
                (signal.value, total_weight)
            })
            .collect()
    }

    /// Get weight for signal type
    fn get_signal_type_weight(&self, signal_type: SignalType) -> f64 {
        match signal_type {
            SignalType::Sentiment => self.config.signal_weights.sentiment,
            SignalType::Probability => self.config.signal_weights.probability,
            SignalType::Narrative => self.config.signal_weights.narrative,
            SignalType::ModelForecast => self.config.signal_weights.model_forecast,
            SignalType::ConsensusMetric => self.config.signal_weights.consensus_metric,
        }
    }

    /// Calculate weighted average of signals
    fn calculate_weighted_average(&self, weighted_signals: &[(f64, f64)]) -> f64 {
        if weighted_signals.is_empty() {
            return 0.0;
        }

        let total_weight: f64 = weighted_signals.iter().map(|(_, w)| w).sum();
        if total_weight == 0.0 {
            return 0.0;
        }

        let weighted_sum: f64 = weighted_signals
            .iter()
            .map(|(value, weight)| value * weight)
            .sum();

        weighted_sum / total_weight
    }

    /// Calculate velocity (rate of change)
    fn calculate_velocity(&self, current_value: f64) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }

        // Look back over smoothing window
        let window_size = (self.config.smoothing_window / 60).max(1) as usize;
        let lookback = self.history.len().saturating_sub(window_size);
        
        if lookback >= self.history.len() {
            return 0.0;
        }

        let past_value = self.history[lookback].value;
        let time_diff = (self.history.len() - lookback) as f64;

        if time_diff == 0.0 {
            return 0.0;
        }

        (current_value - past_value) / time_diff
    }

    /// Calculate volatility from signal variance
    fn calculate_volatility(&self, signals: &[BeliefSignal]) -> f64 {
        if signals.len() < 2 {
            return 0.0;
        }

        let mean = signals.iter().map(|s| s.value).sum::<f64>() / signals.len() as f64;
        let variance = signals
            .iter()
            .map(|s| (s.value - mean).powi(2))
            .sum::<f64>()
            / signals.len() as f64;

        variance.sqrt()
    }

    /// Calculate confidence score
    fn calculate_confidence(&self, signals: &[BeliefSignal]) -> f64 {
        let signal_count = signals.len() as u32;
        
        // Confidence increases with signal count
        let count_factor = (signal_count as f64 / self.config.min_signal_count as f64).min(1.0);

        // Confidence decreases with high volatility
        let volatility = self.calculate_volatility(signals);
        let volatility_factor = (1.0 - volatility).max(0.0);

        // Combined confidence
        (count_factor * 0.6 + volatility_factor * 0.4).min(1.0)
    }

    /// Apply temporal decay to historical BSI values
    pub fn apply_decay(&mut self) {
        for bsi in &mut self.history {
            bsi.value *= self.config.decay_factor;
            bsi.velocity *= self.config.decay_factor;
        }
    }

    /// Get historical BSI values
    pub fn get_history(&self) -> &[BeliefStateIndex] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::belief::SignalType;

    fn create_test_signal(value: f64, signal_type: SignalType) -> BeliefSignal {
        BeliefSignal {
            source: "test".to_string(),
            signal_type,
            value,
            weight: 1.0,
            timestamp: chrono::Utc::now().timestamp(),
            metadata: vec![],
        }
    }

    #[test]
    fn test_bsi_calculation() {
        let config = BsiConfig::default();
        let mut calculator = BsiCalculator::new(config);

        let signals = vec![
            create_test_signal(0.5, SignalType::Sentiment),
            create_test_signal(0.6, SignalType::Probability),
            create_test_signal(0.4, SignalType::Narrative),
        ];

        let bsi = calculator.calculate(&signals, "BTC".to_string());

        assert!(bsi.value >= 0.0 && bsi.value <= 1.0);
        assert_eq!(bsi.signal_count, 3);
        assert!(bsi.confidence > 0.0);
    }

    #[test]
    fn test_outlier_filtering() {
        let config = BsiConfig::default();
        let calculator = BsiCalculator::new(config);

        let signals = vec![
            create_test_signal(0.5, SignalType::Sentiment),
            create_test_signal(0.6, SignalType::Sentiment),
            create_test_signal(10.0, SignalType::Sentiment), // Outlier
        ];

        let filtered = calculator.filter_outliers(&signals);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_velocity_calculation() {
        let config = BsiConfig::default();
        let mut calculator = BsiCalculator::new(config);

        // First calculation
        let signals1 = vec![create_test_signal(0.3, SignalType::Sentiment)];
        calculator.calculate(&signals1, "BTC".to_string());

        // Second calculation with higher value
        let signals2 = vec![create_test_signal(0.7, SignalType::Sentiment)];
        let bsi = calculator.calculate(&signals2, "BTC".to_string());

        assert!(bsi.velocity > 0.0); // Positive velocity
    }
}
