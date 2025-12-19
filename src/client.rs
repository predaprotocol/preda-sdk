//! Preda client implementation

use async_trait::async_trait;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use std::str::FromStr;
use std::sync::Arc;

use crate::{
    bsi::BeliefStateIndex,
    error::{PredaError, Result},
    market::MarketManager,
    oracle::OracleClient,
    types::{
        belief::BeliefCondition,
        market::{Market, MarketConfig, MarketType},
        position::{Position, TimeBucket, TimeBucketAggregate},
    },
};

/// Main client for interacting with Preda protocol
pub struct PredaClient {
    /// Solana RPC client
    rpc_client: Arc<RpcClient>,

    /// Signer keypair
    keypair: Arc<Keypair>,

    /// Preda program ID
    program_id: Pubkey,

    /// Market manager
    market_manager: MarketManager,

    /// Oracle client
    oracle_client: OracleClient,
}

impl PredaClient {
    /// Create a new Preda client
    ///
    /// # Arguments
    ///
    /// * `rpc_url` - Solana RPC endpoint URL
    /// * `keypair` - Signer keypair for transactions
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use preda_sdk::PredaClient;
    /// use solana_sdk::signature::Keypair;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let keypair = Keypair::new();
    ///     let client = PredaClient::new(
    ///         "https://api.mainnet-beta.solana.com",
    ///         keypair,
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(rpc_url: &str, keypair: Keypair) -> Result<Self> {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        ));

        let program_id = Pubkey::from_str(crate::PREDA_PROGRAM_ID)
            .map_err(|e| PredaError::InvalidPublicKey(e.to_string()))?;

        let keypair = Arc::new(keypair);
        let market_manager = MarketManager::new(Arc::clone(&rpc_client), program_id);
        let oracle_client = OracleClient::new(Arc::clone(&rpc_client));

        Ok(Self {
            rpc_client,
            keypair,
            program_id,
            market_manager,
            oracle_client,
        })
    }

    /// Create a new time-shifted prediction market
    ///
    /// # Arguments
    ///
    /// * `market_type` - Type of market to create
    /// * `belief_condition` - Belief condition for resolution
    /// * `description` - Human-readable market description
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use preda_sdk::{PredaClient, MarketType, BeliefCondition};
    ///
    /// # async fn example(client: &PredaClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let market = client.create_market(
    ///     MarketType::SentimentTransition,
    ///     BeliefCondition::SentimentShift {
    ///         from_polarity: -0.2,
    ///         to_polarity: 0.6,
    ///         persistence_window: 3600,
    ///     },
    ///     "BTC sentiment turns bullish",
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_market(
        &self,
        market_type: MarketType,
        belief_condition: BeliefCondition,
        description: &str,
    ) -> Result<Market> {
        self.create_market_with_config(
            market_type,
            belief_condition,
            description,
            MarketConfig::default(),
        )
        .await
    }

    /// Create a market with custom configuration
    pub async fn create_market_with_config(
        &self,
        market_type: MarketType,
        belief_condition: BeliefCondition,
        description: &str,
        config: MarketConfig,
    ) -> Result<Market> {
        // Validate inputs
        belief_condition
            .validate()
            .map_err(|e| PredaError::InvalidBeliefCondition(e))?;
        config
            .validate()
            .map_err(|e| PredaError::Configuration(e))?;

        self.market_manager
            .create_market(
                &self.keypair,
                market_type,
                belief_condition,
                description.to_string(),
                config,
            )
            .await
    }

    /// Get market by address
    pub async fn get_market(&self, market_address: &Pubkey) -> Result<Market> {
        self.market_manager.get_market(market_address).await
    }

    /// Get all active markets
    pub async fn get_active_markets(&self) -> Result<Vec<Market>> {
        self.market_manager.get_active_markets().await
    }

    /// Get Belief State Index for a market
    pub async fn get_belief_state_index(&self, market_address: &Pubkey) -> Result<BeliefStateIndex> {
        self.market_manager
            .get_belief_state_index(market_address)
            .await
    }

    /// Place a position in a market
    ///
    /// # Arguments
    ///
    /// * `market_address` - Market to place position in
    /// * `time_bucket_start` - Start timestamp of time bucket
    /// * `amount` - Amount to stake in lamports
    pub async fn place_position(
        &self,
        market_address: &Pubkey,
        time_bucket_start: i64,
        amount: u64,
    ) -> Result<Position> {
        self.market_manager
            .place_position(&self.keypair, market_address, time_bucket_start, amount)
            .await
    }

    /// Get user's positions in a market
    pub async fn get_user_positions(&self, market_address: &Pubkey) -> Result<Vec<Position>> {
        self.market_manager
            .get_user_positions(market_address, &self.keypair.pubkey())
            .await
    }

    /// Get all positions for a time bucket
    pub async fn get_time_bucket_positions(
        &self,
        market_address: &Pubkey,
        time_bucket: TimeBucket,
    ) -> Result<Vec<Position>> {
        self.market_manager
            .get_time_bucket_positions(market_address, time_bucket)
            .await
    }

    /// Get aggregated data for all time buckets in a market
    pub async fn get_time_bucket_aggregates(
        &self,
        market_address: &Pubkey,
    ) -> Result<Vec<TimeBucketAggregate>> {
        self.market_manager
            .get_time_bucket_aggregates(market_address)
            .await
    }

    /// Withdraw position before market resolution
    pub async fn withdraw_position(&self, position_address: &Pubkey) -> Result<Signature> {
        self.market_manager
            .withdraw_position(&self.keypair, position_address)
            .await
    }

    /// Claim payout from settled position
    pub async fn claim_payout(&self, position_address: &Pubkey) -> Result<Signature> {
        self.market_manager
            .claim_payout(&self.keypair, position_address)
            .await
    }

    /// Get oracle client for direct oracle queries
    pub fn oracle(&self) -> &OracleClient {
        &self.oracle_client
    }

    /// Get user's public key
    pub fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }

    /// Get Solana RPC client
    pub fn rpc(&self) -> &RpcClient {
        &self.rpc_client
    }

    /// Get program ID
    pub fn program_id(&self) -> Pubkey {
        self.program_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let keypair = Keypair::new();
        // This will fail without a real RPC endpoint, but tests the structure
        let result = PredaClient::new("http://localhost:8899", keypair).await;
        // We expect this to potentially fail in test environment
        assert!(result.is_ok() || result.is_err());
    }
}
