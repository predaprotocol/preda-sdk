//! Forecast aggregation oracle implementation

use async_trait::async_trait;
use crate::error::{PredaError, Result};
use crate::oracle::Oracle;
use crate::types::belief::{BeliefSignal, SignalType};

/// Forecast oracle for aggregating probabilistic forecasts
pub struct ForecastOracle {
    api_endpoint: String,
}

impl ForecastOracle {
    pub fn new() -> Self {
        Self {
            api_endpoint: "https://api.preda.io/forecast".to_string(),
        }
    }

    pub fn with_endpoint(endpoint: String) -> Self {
        Self { api_endpoint: endpoint }
    }
}

#[async_trait]
impl Oracle for ForecastOracle {
    async fn query(&self, domain: &str) -> Result<BeliefSignal> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}", &self.api_endpoint, domain);

        let response = client.get(&url).send().await
            .map_err(|e| PredaError::Oracle(format!("Forecast API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(PredaError::Oracle(format!("Forecast API returned status: {}", response.status())));
        }

        let data: serde_json::Value = response.json().await
            .map_err(|e| PredaError::Oracle(format!("Failed to parse forecast response: {}", e)))?;

        let forecast_value = data.get("probability")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| PredaError::Oracle("Invalid forecast data".to_string()))?;

        Ok(BeliefSignal {
            source: "forecast_oracle".to_string(),
            signal_type: SignalType::Probability,
            value: forecast_value,
            weight: 1.2,
            timestamp: chrono::Utc::now().timestamp(),
            metadata: vec![
                ("domain".to_string(), domain.to_string()),
                ("oracle".to_string(), "forecast".to_string()),
            ],
        })
    }

    fn name(&self) -> &str {
        "Forecast Oracle"
    }

    fn update_frequency(&self) -> u64 {
        300 // 5 minutes
    }
}

impl Default for ForecastOracle {
    fn default() -> Self {
        Self::new()
    }
}
