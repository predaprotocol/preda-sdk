//! Sentiment oracle implementation

use async_trait::async_trait;
use crate::error::{PredaError, Result};
use crate::oracle::Oracle;
use crate::types::belief::{BeliefSignal, SignalType};

/// Sentiment oracle for capturing social sentiment
pub struct SentimentOracle {
    api_endpoint: String,
}

impl SentimentOracle {
    /// Create a new sentiment oracle
    pub fn new() -> Self {
        Self {
            api_endpoint: "https://api.preda.io/sentiment".to_string(),
        }
    }

    /// Create with custom endpoint
    pub fn with_endpoint(endpoint: String) -> Self {
        Self {
            api_endpoint: endpoint,
        }
    }

    /// Parse sentiment data from API response
    fn parse_sentiment(&self, data: &serde_json::Value) -> Result<f64> {
        data.get("sentiment_score")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| PredaError::Oracle("Invalid sentiment data".to_string()))
    }
}

#[async_trait]
impl Oracle for SentimentOracle {
    async fn query(&self, domain: &str) -> Result<BeliefSignal> {
        // In production, this would make an HTTP request to the sentiment API
        // For now, we'll return a mock signal
        
        let client = reqwest::Client::new();
        let url = format!("{}/{}",  &self.api_endpoint, domain);

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| PredaError::Oracle(format!("Sentiment API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(PredaError::Oracle(format!(
                "Sentiment API returned status: {}",
                response.status()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| PredaError::Oracle(format!("Failed to parse sentiment response: {}", e)))?;

        let sentiment_value = self.parse_sentiment(&data)?;

        Ok(BeliefSignal {
            source: "sentiment_oracle".to_string(),
            signal_type: SignalType::Sentiment,
            value: sentiment_value,
            weight: 1.0,
            timestamp: chrono::Utc::now().timestamp(),
            metadata: vec![
                ("domain".to_string(), domain.to_string()),
                ("oracle".to_string(), "sentiment".to_string()),
            ],
        })
    }

    fn name(&self) -> &str {
        "Sentiment Oracle"
    }

    fn update_frequency(&self) -> u64 {
        300 // 5 minutes
    }
}

impl Default for SentimentOracle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentiment_oracle_creation() {
        let oracle = SentimentOracle::new();
        assert_eq!(oracle.name(), "Sentiment Oracle");
        assert_eq!(oracle.update_frequency(), 300);
    }

    #[test]
    fn test_parse_sentiment() {
        let oracle = SentimentOracle::new();
        let data = serde_json::json!({
            "sentiment_score": 0.75
        });

        let result = oracle.parse_sentiment(&data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.75);
    }
}
