//! Volatility-aware settlement logic

use crate::types::{
    market::{Market, SettlementCurve},
    position::{Position, TimeBucket},
    belief::BeliefInflection,
};

/// Settlement calculator for volatility-aware payouts
pub struct SettlementCalculator;

impl SettlementCalculator {
    /// Calculate payout for a position
    pub fn calculate_payout(
        market: &Market,
        position: &Position,
        inflection: &BeliefInflection,
    ) -> u64 {
        let distance = position.time_bucket.distance_from(inflection.timestamp);
        
        let payout_multiplier = match market.config.settlement_curve {
            SettlementCurve::Linear => Self::linear_payout(distance, inflection.sharpness),
            SettlementCurve::Exponential => Self::exponential_payout(distance, inflection.sharpness),
            SettlementCurve::Gaussian => Self::gaussian_payout(distance, inflection.sharpness),
            SettlementCurve::Custom => Self::custom_payout(distance, inflection.sharpness),
        };

        let base_payout = position.amount as f64 * payout_multiplier;
        let volatility_adjusted = base_payout * market.config.volatility_factor;
        
        volatility_adjusted as u64
    }

    /// Linear payout curve
    fn linear_payout(distance: i64, sharpness: f64) -> f64 {
        if distance == 0 {
            return 2.0; // 2x payout for exact match
        }
        
        let decay_rate = 0.0001;
        let multiplier = 2.0 - (distance.abs() as f64 * decay_rate);
        multiplier.max(0.0) * (1.0 + sharpness * 0.5)
    }

    /// Exponential decay payout curve
    fn exponential_payout(distance: i64, sharpness: f64) -> f64 {
        let decay_constant = 0.001;
        let multiplier = 2.0 * (-decay_constant * distance.abs() as f64).exp();
        multiplier * (1.0 + sharpness * 0.5)
    }

    /// Gaussian distribution payout curve
    fn gaussian_payout(distance: i64, sharpness: f64) -> f64 {
        let sigma: f64 = 3600.0; // 1 hour standard deviation
        let exponent = -(distance as f64).powi(2) / (2.0 * sigma.powi(2));
        let multiplier = 2.0 * exponent.exp();
        multiplier * (1.0 + sharpness * 0.5)
    }

    /// Custom payout curve
    fn custom_payout(distance: i64, sharpness: f64) -> f64 {
        // Placeholder for custom curves
        Self::gaussian_payout(distance, sharpness)
    }

    /// Calculate total pool payout
    pub fn calculate_pool_distribution(
        market: &Market,
        winning_positions: &[Position],
        inflection: &BeliefInflection,
    ) -> Vec<(Position, u64)> {
        let total_pool = market.total_value_locked;
        
        // Calculate individual payouts
        let payouts: Vec<(Position, u64)> = winning_positions
            .iter()
            .map(|pos| {
                let payout = Self::calculate_payout(market, pos, inflection);
                (pos.clone(), payout)
            })
            .collect();

        // Normalize to total pool
        let total_payout: u64 = payouts.iter().map(|(_, p)| p).sum();
        
        if total_payout == 0 {
            return payouts;
        }

        payouts
            .into_iter()
            .map(|(pos, payout)| {
                let normalized = (payout as f64 / total_payout as f64 * total_pool as f64) as u64;
                (pos, normalized)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::belief::InflectionType;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_linear_payout() {
        let payout_exact = SettlementCalculator::linear_payout(0, 0.5);
        assert!(payout_exact > 2.0);

        let payout_near = SettlementCalculator::linear_payout(100, 0.5);
        assert!(payout_near < payout_exact);
    }

    #[test]
    fn test_gaussian_payout() {
        let payout_exact = SettlementCalculator::gaussian_payout(0, 0.5);
        assert!(payout_exact > 2.0);

        let payout_far = SettlementCalculator::gaussian_payout(7200, 0.5);
        assert!(payout_far < payout_exact);
    }
}
