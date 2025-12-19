//! Position-related type definitions

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// User position in a time-shifted prediction market
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Position {
    /// Position address
    pub address: Pubkey,

    /// Market address
    pub market: Pubkey,

    /// Position owner
    pub owner: Pubkey,

    /// Time bucket this position targets
    pub time_bucket: TimeBucket,

    /// Amount staked (lamports)
    pub amount: u64,

    /// Position status
    pub status: PositionStatus,

    /// Creation timestamp
    pub created_at: i64,

    /// Settlement timestamp (if settled)
    pub settled_at: Option<i64>,

    /// Payout amount (if settled)
    pub payout: Option<u64>,
}

/// Time bucket for position allocation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub struct TimeBucket {
    /// Start timestamp (Unix)
    pub start: i64,

    /// End timestamp (Unix)
    pub end: i64,
}

/// Position status in lifecycle
#[derive(Debug, Clone, Copy, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum PositionStatus {
    /// Position is active
    Active,

    /// Position won (inflection in time bucket)
    Won,

    /// Position lost (inflection outside time bucket)
    Lost,

    /// Position partially won (volatility-adjusted)
    PartialWin,

    /// Position withdrawn before resolution
    Withdrawn,

    /// Market expired without resolution
    Expired,
}

/// Aggregated position data for a time bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBucketAggregate {
    /// Time bucket
    pub time_bucket: TimeBucket,

    /// Total amount staked in this bucket
    pub total_staked: u64,

    /// Number of positions in this bucket
    pub position_count: u32,

    /// Implied probability (based on stake distribution)
    pub implied_probability: f64,

    /// Average position size
    pub avg_position_size: u64,
}

impl Position {
    /// Check if position is active
    pub fn is_active(&self) -> bool {
        self.status == PositionStatus::Active
    }

    /// Check if position has been settled
    pub fn is_settled(&self) -> bool {
        matches!(
            self.status,
            PositionStatus::Won | PositionStatus::Lost | PositionStatus::PartialWin
        )
    }

    /// Check if position won
    pub fn is_winner(&self) -> bool {
        matches!(self.status, PositionStatus::Won | PositionStatus::PartialWin)
    }

    /// Calculate return on investment (if settled)
    pub fn roi(&self) -> Option<f64> {
        self.payout.map(|payout| {
            if self.amount == 0 {
                0.0
            } else {
                ((payout as f64 - self.amount as f64) / self.amount as f64) * 100.0
            }
        })
    }

    /// Get position age in seconds
    pub fn age(&self, current_time: i64) -> i64 {
        current_time - self.created_at
    }
}

impl TimeBucket {
    /// Create a new time bucket
    pub fn new(start: i64, end: i64) -> Result<Self, String> {
        if start >= end {
            return Err("Start time must be before end time".to_string());
        }
        Ok(Self { start, end })
    }

    /// Create time bucket from start and duration
    pub fn from_duration(start: i64, duration: u64) -> Self {
        Self {
            start,
            end: start + duration as i64,
        }
    }

    /// Check if timestamp falls within this bucket
    pub fn contains(&self, timestamp: i64) -> bool {
        timestamp >= self.start && timestamp < self.end
    }

    /// Get bucket duration in seconds
    pub fn duration(&self) -> u64 {
        (self.end - self.start) as u64
    }

    /// Get bucket midpoint
    pub fn midpoint(&self) -> i64 {
        (self.start + self.end) / 2
    }

    /// Check if this bucket overlaps with another
    pub fn overlaps(&self, other: &TimeBucket) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Calculate distance from timestamp to bucket (0 if inside)
    pub fn distance_from(&self, timestamp: i64) -> i64 {
        if self.contains(timestamp) {
            0
        } else if timestamp < self.start {
            self.start - timestamp
        } else {
            timestamp - self.end
        }
    }
}

impl TimeBucketAggregate {
    /// Calculate implied probability from stake distribution
    pub fn calculate_implied_probability(bucket_stake: u64, total_market_stake: u64) -> f64 {
        if total_market_stake == 0 {
            0.0
        } else {
            bucket_stake as f64 / total_market_stake as f64
        }
    }

    /// Check if this bucket has significant stake
    pub fn is_significant(&self, threshold: f64) -> bool {
        self.implied_probability >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_bucket_creation() {
        let bucket = TimeBucket::new(1000, 2000).unwrap();
        assert_eq!(bucket.duration(), 1000);
        assert_eq!(bucket.midpoint(), 1500);

        let invalid = TimeBucket::new(2000, 1000);
        assert!(invalid.is_err());
    }

    #[test]
    fn test_time_bucket_contains() {
        let bucket = TimeBucket::new(1000, 2000).unwrap();
        assert!(bucket.contains(1500));
        assert!(!bucket.contains(500));
        assert!(!bucket.contains(2500));
    }

    #[test]
    fn test_time_bucket_distance() {
        let bucket = TimeBucket::new(1000, 2000).unwrap();
        assert_eq!(bucket.distance_from(1500), 0);
        assert_eq!(bucket.distance_from(500), 500);
        assert_eq!(bucket.distance_from(2500), 500);
    }

    #[test]
    fn test_position_roi() {
        let mut position = Position {
            address: Pubkey::new_unique(),
            market: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            time_bucket: TimeBucket::new(1000, 2000).unwrap(),
            amount: 1_000_000,
            status: PositionStatus::Active,
            created_at: 0,
            settled_at: None,
            payout: None,
        };

        assert!(position.roi().is_none());

        position.payout = Some(2_000_000);
        assert_eq!(position.roi(), Some(100.0)); // 100% ROI
    }

    #[test]
    fn test_implied_probability() {
        let prob = TimeBucketAggregate::calculate_implied_probability(250, 1000);
        assert_eq!(prob, 0.25);
    }
}
