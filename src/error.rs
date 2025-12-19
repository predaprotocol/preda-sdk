//! Error types for the Preda SDK

use thiserror::Error;

/// Result type alias for Preda SDK operations
pub type Result<T> = std::result::Result<T, PredaError>;

/// Comprehensive error types for Preda SDK
#[derive(Error, Debug)]
pub enum PredaError {
    /// Solana client errors
    #[error("Solana client error: {0}")]
    SolanaClient(#[from] solana_client::client_error::ClientError),

    /// Solana SDK errors
    #[error("Solana SDK error: {0}")]
    SolanaSdk(String),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Borsh serialization errors
    #[error("Borsh serialization error: {0}")]
    BorshSerialization(#[from] std::io::Error),

    /// HTTP request errors
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    /// Market not found
    #[error("Market not found: {0}")]
    MarketNotFound(String),

    /// Invalid market state
    #[error("Invalid market state: expected {expected}, got {actual}")]
    InvalidMarketState { expected: String, actual: String },

    /// Invalid belief condition
    #[error("Invalid belief condition: {0}")]
    InvalidBeliefCondition(String),

    /// Oracle error
    #[error("Oracle error: {0}")]
    Oracle(String),

    /// BSI calculation error
    #[error("BSI calculation error: {0}")]
    BsiCalculation(String),

    /// Insufficient funds
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },

    /// Position not found
    #[error("Position not found for time bucket: {0}")]
    PositionNotFound(i64),

    /// Market already resolved
    #[error("Market already resolved at timestamp: {0}")]
    MarketAlreadyResolved(i64),

    /// Invalid time bucket
    #[error("Invalid time bucket: {0}")]
    InvalidTimeBucket(String),

    /// Threshold not met
    #[error("Belief threshold not met: current {current}, required {required}")]
    ThresholdNotMet { current: f64, required: f64 },

    /// Persistence window not satisfied
    #[error("Persistence window not satisfied: duration {duration}s, required {required}s")]
    PersistenceNotSatisfied { duration: u64, required: u64 },

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Invalid public key
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    /// Program error
    #[error("Program error: {0}")]
    Program(String),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

impl From<solana_sdk::pubkey::ParsePubkeyError> for PredaError {
    fn from(err: solana_sdk::pubkey::ParsePubkeyError) -> Self {
        PredaError::InvalidPublicKey(err.to_string())
    }
}

impl From<anyhow::Error> for PredaError {
    fn from(err: anyhow::Error) -> Self {
        PredaError::Generic(err.to_string())
    }
}
