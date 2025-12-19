//! Oracle integration module

pub mod consensus;
pub mod forecast;
pub mod narrative;
pub mod sentiment;

use async_trait::async_trait;
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;

use crate::error::Result;
use crate::types::belief::BeliefSignal;

/// Oracle client for querying belief signals
pub struct OracleClient {
    rpc_client: Arc<RpcClient>,
    sentiment_oracle: sentiment::SentimentOracle,
    narrative_oracle: narrative::NarrativeOracle,
    forecast_oracle: forecast::ForecastOracle,
    consensus_oracle: consensus::ConsensusOracle,
}

impl OracleClient {
    /// Create a new oracle client
    pub fn new(rpc_client: Arc<RpcClient>) -> Self {
        Self {
            rpc_client: Arc::clone(&rpc_client),
            sentiment_oracle: sentiment::SentimentOracle::new(),
            narrative_oracle: narrative::NarrativeOracle::new(),
            forecast_oracle: forecast::ForecastOracle::new(),
            consensus_oracle: consensus::ConsensusOracle::new(),
        }
    }

    /// Query sentiment oracle
    pub async fn query_sentiment(&self, domain: &str) -> Result<BeliefSignal> {
        self.sentiment_oracle.query(domain).await
    }

    /// Query narrative oracle
    pub async fn query_narrative(&self, domain: &str) -> Result<BeliefSignal> {
        self.narrative_oracle.query(domain).await
    }

    /// Query forecast aggregation oracle
    pub async fn query_forecast(&self, domain: &str) -> Result<BeliefSignal> {
        self.forecast_oracle.query(domain).await
    }

    /// Query AI consensus oracle
    pub async fn query_consensus(&self, domain: &str) -> Result<BeliefSignal> {
        self.consensus_oracle.query(domain).await
    }

    /// Query all oracles for a domain
    pub async fn query_all(&self, domain: &str) -> Result<Vec<BeliefSignal>> {
        let mut signals = Vec::new();

        if let Ok(signal) = self.query_sentiment(domain).await {
            signals.push(signal);
        }

        if let Ok(signal) = self.query_narrative(domain).await {
            signals.push(signal);
        }

        if let Ok(signal) = self.query_forecast(domain).await {
            signals.push(signal);
        }

        if let Ok(signal) = self.query_consensus(domain).await {
            signals.push(signal);
        }

        Ok(signals)
    }
}

/// Trait for oracle implementations
#[async_trait]
pub trait Oracle: Send + Sync {
    /// Query the oracle for a belief signal
    async fn query(&self, domain: &str) -> Result<BeliefSignal>;

    /// Get oracle name
    fn name(&self) -> &str;

    /// Get oracle update frequency (seconds)
    fn update_frequency(&self) -> u64;
}
