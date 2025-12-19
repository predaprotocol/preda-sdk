//! AI consensus oracle implementation

use async_trait::async_trait;
use crate::error::{PredaError, Result};
use crate::oracle::Oracle;
use crate::types::belief::{BeliefSignal, SignalType};

/// Consensus oracle for measuring AI model agreement
pub struct ConsensusOracle {
    api_endpoint: String,
}

impl ConsensusOracle {
    pub fn new() -> Self {
        Self {
            api_endpoint: "https://api.preda.io/consensus".to_string(),
        }
    }

    pub fn with_endpoint(endpoint: String) -> Self {
        Self { api_endpoint: endpoint }
    }
}

#[async_trait]
impl Oracle for ConsensusOracle {
    async fn query(&self, domain: &str) -> Result<BeliefSignal> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}", &self.api_endpoint, domain);

        let response = client.get(&url).send().await
            .map_err(|e| PredaError::Oracle(format!("Consensus API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(PredaError::Oracle(format!("Consensus API returned status: {}", response.status())));
        }

        let data: serde_json::Value = response.json().await
            .map_err(|e| PredaError::Oracle(format!("Failed to parse consensus response: {}", e)))?;

        let consensus_value = data.get("consensus_score")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| PredaError::Oracle("Invalid consensus data".to_string()))?;

        Ok(BeliefSignal {
            source: "consensus_oracle".to_string(),
            signal_type: SignalType::ConsensusMetric,
            value: consensus_value,
            weight: 1.3,
            timestamp: chrono::Utc::now().timestamp(),
            metadata: vec![
                ("domain".to_string(), domain.to_string()),
                ("oracle".to_string(), "consensus".to_string()),
            ],
        })
    }

    fn name(&self) -> &str {
        "AI Consensus Oracle"
    }

    fn update_frequency(&self) -> u64 {
        300 // 5 minutes
    }
}

impl Default for ConsensusOracle {
    fn default() -> Self {
        Self::new()
    }
}
