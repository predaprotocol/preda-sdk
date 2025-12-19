//! Signal aggregator for combining multiple oracle inputs

use crate::types::belief::{BeliefSignal, SignalType};
use std::collections::HashMap;

/// Aggregates belief signals from multiple sources
pub struct SignalAggregator {
    /// Buffered signals by source
    signal_buffer: HashMap<String, Vec<BeliefSignal>>,

    /// Maximum buffer size per source
    max_buffer_size: usize,
}

impl SignalAggregator {
    /// Create a new signal aggregator
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            signal_buffer: HashMap::new(),
            max_buffer_size,
        }
    }

    /// Add a signal to the buffer
    pub fn add_signal(&mut self, signal: BeliefSignal) {
        let source = signal.source.clone();
        let buffer = self.signal_buffer.entry(source).or_insert_with(Vec::new);

        buffer.push(signal);

        // Trim buffer if too large
        if buffer.len() > self.max_buffer_size {
            buffer.remove(0);
        }
    }

    /// Add multiple signals
    pub fn add_signals(&mut self, signals: Vec<BeliefSignal>) {
        for signal in signals {
            self.add_signal(signal);
        }
    }

    /// Get all signals from buffer
    pub fn get_all_signals(&self) -> Vec<BeliefSignal> {
        self.signal_buffer
            .values()
            .flat_map(|signals| signals.iter().cloned())
            .collect()
    }

    /// Get signals by type
    pub fn get_signals_by_type(&self, signal_type: SignalType) -> Vec<BeliefSignal> {
        self.signal_buffer
            .values()
            .flat_map(|signals| signals.iter())
            .filter(|s| s.signal_type == signal_type)
            .cloned()
            .collect()
    }

    /// Get signals from specific source
    pub fn get_signals_by_source(&self, source: &str) -> Vec<BeliefSignal> {
        self.signal_buffer
            .get(source)
            .map(|signals| signals.clone())
            .unwrap_or_default()
    }

    /// Get recent signals within time window
    pub fn get_recent_signals(&self, window_seconds: i64) -> Vec<BeliefSignal> {
        let cutoff = chrono::Utc::now().timestamp() - window_seconds;

        self.signal_buffer
            .values()
            .flat_map(|signals| signals.iter())
            .filter(|s| s.timestamp >= cutoff)
            .cloned()
            .collect()
    }

    /// Get average signal value by type
    pub fn get_average_by_type(&self, signal_type: SignalType) -> Option<f64> {
        let signals = self.get_signals_by_type(signal_type);
        
        if signals.is_empty() {
            return None;
        }

        let sum: f64 = signals.iter().map(|s| s.value).sum();
        Some(sum / signals.len() as f64)
    }

    /// Get signal diversity (number of unique sources)
    pub fn get_source_diversity(&self) -> usize {
        self.signal_buffer.len()
    }

    /// Get total signal count
    pub fn get_total_signal_count(&self) -> usize {
        self.signal_buffer.values().map(|v| v.len()).sum()
    }

    /// Clear all signals
    pub fn clear(&mut self) {
        self.signal_buffer.clear();
    }

    /// Clear signals older than specified time
    pub fn clear_old_signals(&mut self, max_age_seconds: i64) {
        let cutoff = chrono::Utc::now().timestamp() - max_age_seconds;

        for signals in self.signal_buffer.values_mut() {
            signals.retain(|s| s.timestamp >= cutoff);
        }

        // Remove empty buffers
        self.signal_buffer.retain(|_, signals| !signals.is_empty());
    }

    /// Get signal statistics
    pub fn get_statistics(&self) -> SignalStatistics {
        let all_signals = self.get_all_signals();

        if all_signals.is_empty() {
            return SignalStatistics::default();
        }

        let values: Vec<f64> = all_signals.iter().map(|s| s.value).collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        
        let variance = values
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>()
            / values.len() as f64;
        
        let std_dev = variance.sqrt();
        
        let mut sorted_values = values.clone();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let median = if sorted_values.len() % 2 == 0 {
            let mid = sorted_values.len() / 2;
            (sorted_values[mid - 1] + sorted_values[mid]) / 2.0
        } else {
            sorted_values[sorted_values.len() / 2]
        };

        SignalStatistics {
            count: all_signals.len(),
            mean,
            median,
            std_dev,
            min: *sorted_values.first().unwrap(),
            max: *sorted_values.last().unwrap(),
            source_count: self.get_source_diversity(),
        }
    }
}

/// Signal statistics
#[derive(Debug, Clone, Default)]
pub struct SignalStatistics {
    pub count: usize,
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub source_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_signal(source: &str, value: f64, signal_type: SignalType) -> BeliefSignal {
        BeliefSignal {
            source: source.to_string(),
            signal_type,
            value,
            weight: 1.0,
            timestamp: chrono::Utc::now().timestamp(),
            metadata: vec![],
        }
    }

    #[test]
    fn test_signal_aggregation() {
        let mut aggregator = SignalAggregator::new(100);

        aggregator.add_signal(create_test_signal("oracle1", 0.5, SignalType::Sentiment));
        aggregator.add_signal(create_test_signal("oracle2", 0.6, SignalType::Sentiment));

        assert_eq!(aggregator.get_total_signal_count(), 2);
        assert_eq!(aggregator.get_source_diversity(), 2);
    }

    #[test]
    fn test_signal_filtering() {
        let mut aggregator = SignalAggregator::new(100);

        aggregator.add_signal(create_test_signal("oracle1", 0.5, SignalType::Sentiment));
        aggregator.add_signal(create_test_signal("oracle2", 0.6, SignalType::Probability));

        let sentiment_signals = aggregator.get_signals_by_type(SignalType::Sentiment);
        assert_eq!(sentiment_signals.len(), 1);
    }

    #[test]
    fn test_statistics() {
        let mut aggregator = SignalAggregator::new(100);

        aggregator.add_signal(create_test_signal("oracle1", 0.3, SignalType::Sentiment));
        aggregator.add_signal(create_test_signal("oracle2", 0.5, SignalType::Sentiment));
        aggregator.add_signal(create_test_signal("oracle3", 0.7, SignalType::Sentiment));

        let stats = aggregator.get_statistics();
        assert_eq!(stats.count, 3);
        assert_eq!(stats.median, 0.5);
    }
}
