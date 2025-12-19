//! Narrative oracle implementation

use async_trait::async_trait;
use crate::error::{PredaError, Result};
use crate::oracle::Oracle;
use crate::types::belief::{BeliefSignal, SignalType};

/// Narrative oracle for tracking narrative framing and topic dominance
pub struct NarrativeOracle {
    api_endpoint: String,
}

impl NarrativeOracle {
    pub fn new() -> Self {
        Self {
            api_endpoint: "https://api.preda.io/narrative".to_string(),
        }
    }

    pub fn with_endpoint(endpoint: String) -> Self {
        Self { api_endpoint: endpoint }
    }
}

#[async_trait]
impl Oracle for NarrativeOracle {
    async fn query(&self, domain: &str) -> Result<BeliefSignal> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}", &self.api_endpoint, domain);

        let response = client.get(&url).send().await
            .map_err(|e| PredaError::Oracle(format!("Narrative API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(PredaError::Oracle(format!("Narrative API returned status: {}", response.status())));
        }

        let data: serde_json::Value = response.json().await
            .map_err(|e| PredaError::Oracle(format!("Failed to parse narrative response: {}", e)))?;

        let narrative_value = data.get("narrative_score")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| PredaError::Oracle("Invalid narrative data".to_string()))?;

        Ok(BeliefSignal {
            source: "narrative_oracle".to_string(),
            signal_type: SignalType::Narrative,
            value: narrative_value,
            weight: 0.8,
            timestamp: chrono::Utc::now().timestamp(),
            metadata: vec![
                ("domain".to_string(), domain.to_string()),
                ("oracle".to_string(), "narrative".to_string()),
            ],
        })
    }

    fn name(&self) -> &str {
        "Narrative Oracle"
    }

    fn update_frequency(&self) -> u64 {
        600 // 10 minutes
    }
}

impl Default for NarrativeOracle {
    fn default() -> Self {
        Self::new()
    }
}
