//! Belief monitor for detecting inflection points

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::belief::{BeliefInflection, BeliefStateIndex, InflectionType};
use crate::error::Result;

/// Callback type for inflection events
pub type InflectionCallback = Arc<dyn Fn(BeliefInflection) + Send + Sync>;

/// Monitors belief state for inflection points
pub struct BeliefMonitor {
    /// Historical BSI values
    history: Arc<RwLock<Vec<BeliefStateIndex>>>,

    /// Inflection detection threshold
    threshold: f64,

    /// Minimum persistence duration (seconds)
    min_persistence: u64,

    /// Callbacks for inflection events
    callbacks: Arc<RwLock<Vec<InflectionCallback>>>,
}

impl BeliefMonitor {
    /// Create a new belief monitor
    pub fn new(threshold: f64, min_persistence: u64) -> Self {
        Self {
            history: Arc::new(RwLock::new(Vec::new())),
            threshold,
            min_persistence,
            callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add BSI update to monitor
    pub async fn update(&self, bsi: BeliefStateIndex) -> Result<Option<BeliefInflection>> {
        let mut history = self.history.write().await;
        history.push(bsi.clone());

        // Keep only recent history (last 1000 updates)
        if history.len() > 1000 {
            history.remove(0);
        }

        // Detect inflection
        let inflection = self.detect_inflection(&history, &bsi).await;

        // Trigger callbacks if inflection detected
        if let Some(ref infl) = inflection {
            self.trigger_callbacks(infl.clone()).await;
        }

        Ok(inflection)
    }

    /// Register callback for inflection events
    pub async fn on_inflection<F>(&self, callback: F)
    where
        F: Fn(BeliefInflection) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.write().await;
        callbacks.push(Arc::new(callback));
    }

    /// Detect inflection point
    async fn detect_inflection(
        &self,
        history: &[BeliefStateIndex],
        current: &BeliefStateIndex,
    ) -> Option<BeliefInflection> {
        if history.len() < 3 {
            return None;
        }

        // Check for sentiment reversal
        if let Some(inflection) = self.check_sentiment_reversal(history, current) {
            return Some(inflection);
        }

        // Check for threshold crossing
        if let Some(inflection) = self.check_threshold_crossing(history, current) {
            return Some(inflection);
        }

        // Check for velocity spike
        if let Some(inflection) = self.check_velocity_spike(history, current) {
            return Some(inflection);
        }

        None
    }

    /// Check for sentiment reversal
    fn check_sentiment_reversal(
        &self,
        history: &[BeliefStateIndex],
        current: &BeliefStateIndex,
    ) -> Option<BeliefInflection> {
        let lookback = history.len().saturating_sub(10).max(0);
        let past_values: Vec<f64> = history[lookback..].iter().map(|b| b.value).collect();

        // Check if sentiment changed sign
        let past_avg = past_values.iter().sum::<f64>() / past_values.len() as f64;
        
        if (past_avg > 0.0 && current.value < -self.threshold)
            || (past_avg < 0.0 && current.value > self.threshold)
        {
            return Some(BeliefInflection {
                inflection_type: InflectionType::SentimentReversal,
                timestamp: current.last_updated,
                bsi_value: current.value,
                velocity: current.velocity,
                sharpness: (current.value - past_avg).abs(),
                persistence_duration: 0,
                validated: false,
            });
        }

        None
    }

    /// Check for threshold crossing
    fn check_threshold_crossing(
        &self,
        history: &[BeliefStateIndex],
        current: &BeliefStateIndex,
    ) -> Option<BeliefInflection> {
        if history.is_empty() {
            return None;
        }

        let previous = &history[history.len() - 1];

        // Check if crossed threshold
        if (previous.value < self.threshold && current.value >= self.threshold)
            || (previous.value > -self.threshold && current.value <= -self.threshold)
        {
            return Some(BeliefInflection {
                inflection_type: InflectionType::ThresholdCrossing,
                timestamp: current.last_updated,
                bsi_value: current.value,
                velocity: current.velocity,
                sharpness: (current.value - previous.value).abs(),
                persistence_duration: 0,
                validated: false,
            });
        }

        None
    }

    /// Check for velocity spike
    fn check_velocity_spike(
        &self,
        history: &[BeliefStateIndex],
        current: &BeliefStateIndex,
    ) -> Option<BeliefInflection> {
        let lookback = history.len().saturating_sub(10).max(0);
        let past_velocities: Vec<f64> = history[lookback..].iter().map(|b| b.velocity).collect();

        if past_velocities.is_empty() {
            return None;
        }

        let avg_velocity = past_velocities.iter().sum::<f64>() / past_velocities.len() as f64;
        let velocity_threshold = avg_velocity.abs() * 2.0;

        if current.velocity.abs() > velocity_threshold && velocity_threshold > 0.1 {
            return Some(BeliefInflection {
                inflection_type: InflectionType::VelocitySpike,
                timestamp: current.last_updated,
                bsi_value: current.value,
                velocity: current.velocity,
                sharpness: (current.velocity - avg_velocity).abs(),
                persistence_duration: 0,
                validated: false,
            });
        }

        None
    }

    /// Validate inflection persistence
    pub async fn validate_persistence(
        &self,
        inflection: &BeliefInflection,
    ) -> Result<bool> {
        let history = self.history.read().await;
        
        // Find signals after inflection
        let post_inflection: Vec<&BeliefStateIndex> = history
            .iter()
            .filter(|b| b.last_updated >= inflection.timestamp)
            .collect();

        if post_inflection.is_empty() {
            return Ok(false);
        }

        let duration = post_inflection.last().unwrap().last_updated - inflection.timestamp;

        // Check if condition persisted
        let persisted = match inflection.inflection_type {
            InflectionType::SentimentReversal => {
                post_inflection.iter().all(|b| {
                    (inflection.bsi_value > 0.0 && b.value > 0.0)
                        || (inflection.bsi_value < 0.0 && b.value < 0.0)
                })
            }
            InflectionType::ThresholdCrossing => {
                post_inflection.iter().all(|b| {
                    (inflection.bsi_value >= self.threshold && b.value >= self.threshold)
                        || (inflection.bsi_value <= -self.threshold && b.value <= -self.threshold)
                })
            }
            _ => true,
        };

        Ok(persisted && duration as u64 >= self.min_persistence)
    }

    /// Trigger all registered callbacks
    async fn trigger_callbacks(&self, inflection: BeliefInflection) {
        let callbacks = self.callbacks.read().await;
        for callback in callbacks.iter() {
            callback(inflection.clone());
        }
    }

    /// Get current history
    pub async fn get_history(&self) -> Vec<BeliefStateIndex> {
        self.history.read().await.clone()
    }

    /// Clear history
    pub async fn clear_history(&self) {
        self.history.write().await.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_belief_monitor() {
        let monitor = BeliefMonitor::new(0.5, 60);

        let bsi1 = BeliefStateIndex {
            value: 0.3,
            velocity: 0.0,
            volatility: 0.0,
            last_updated: 1000,
            confidence: 0.8,
            signal_count: 5,
            domain: "BTC".to_string(),
        };

        let result = monitor.update(bsi1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_threshold_crossing_detection() {
        let monitor = BeliefMonitor::new(0.5, 60);

        // Below threshold
        let bsi1 = BeliefStateIndex {
            value: 0.3,
            velocity: 0.0,
            volatility: 0.0,
            last_updated: 1000,
            confidence: 0.8,
            signal_count: 5,
            domain: "BTC".to_string(),
        };
        monitor.update(bsi1).await.unwrap();

        // Above threshold - should detect crossing
        let bsi2 = BeliefStateIndex {
            value: 0.6,
            velocity: 0.1,
            volatility: 0.0,
            last_updated: 1100,
            confidence: 0.8,
            signal_count: 5,
            domain: "BTC".to_string(),
        };
        let result = monitor.update(bsi2).await.unwrap();
        assert!(result.is_some());
        
        if let Some(inflection) = result {
            assert_eq!(inflection.inflection_type, InflectionType::ThresholdCrossing);
        }
    }
}
