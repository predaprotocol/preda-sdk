//! Market lifecycle management

use crate::types::market::{Market, MarketState};
use crate::types::belief::BeliefInflection;
use crate::error::Result;

/// Market lifecycle manager
pub struct LifecycleManager;

impl LifecycleManager {
    /// Transition market to monitoring state
    pub fn start_monitoring(market: &mut Market) -> Result<()> {
        if market.state == MarketState::Active {
            market.state = MarketState::Monitoring;
        }
        Ok(())
    }

    /// Handle inflection detection
    pub fn handle_inflection(market: &mut Market, inflection: BeliefInflection) -> Result<()> {
        if market.state == MarketState::Monitoring {
            market.state = MarketState::InflectionDetected;
        }
        Ok(())
    }

    /// Resolve market
    pub fn resolve_market(market: &mut Market, resolution_time: i64) -> Result<()> {
        market.state = MarketState::Resolved;
        market.resolved_at = Some(resolution_time);
        Ok(())
    }

    /// Cancel market
    pub fn cancel_market(market: &mut Market) -> Result<()> {
        market.state = MarketState::Cancelled;
        Ok(())
    }

    /// Expire market
    pub fn expire_market(market: &mut Market) -> Result<()> {
        market.state = MarketState::Expired;
        Ok(())
    }
}
