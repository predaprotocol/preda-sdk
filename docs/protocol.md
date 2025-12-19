# Preda Protocol Specification

## Overview

Preda is a decentralized protocol for time-shifted prediction markets built on Solana. This document specifies the core protocol mechanics, on-chain program structure, and interaction patterns.

## Architecture

### Program Structure

```
preda-program/
├── instructions/
│   ├── create_market.rs
│   ├── place_position.rs
│   ├── update_bsi.rs
│   ├── resolve_market.rs
│   └── claim_payout.rs
├── state/
│   ├── market.rs
│   ├── position.rs
│   └── bsi.rs
└── lib.rs
```

### Account Structure

#### Market Account

```rust
pub struct MarketAccount {
    pub authority: Pubkey,
    pub market_type: MarketType,
    pub belief_condition: BeliefCondition,
    pub state: MarketState,
    pub config: MarketConfig,
    pub bsi: BeliefStateIndex,
    pub total_value_locked: u64,
    pub participant_count: u32,
    pub oracle_addresses: Vec<Pubkey>,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
}
```

#### Position Account

```rust
pub struct PositionAccount {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub time_bucket: TimeBucket,
    pub amount: u64,
    pub status: PositionStatus,
    pub created_at: i64,
}
```

## Instructions

### 1. CreateMarket

Creates a new time-shifted prediction market.

**Accounts:**

- `[signer]` creator
- `[writable]` market (PDA)
- `[]` system_program

**Parameters:**

- `market_type: MarketType`
- `belief_condition: BeliefCondition`
- `description: String`
- `config: MarketConfig`

### 2. PlacePosition

Places a position in a time bucket.

**Accounts:**

- `[signer]` user
- `[writable]` market
- `[writable]` position (PDA)
- `[]` system_program

**Parameters:**

- `time_bucket_start: i64`
- `amount: u64`

### 3. UpdateBSI

Updates the Belief State Index (oracle instruction).

**Accounts:**

- `[signer]` oracle_authority
- `[writable]` market
- `[]` clock

**Parameters:**

- `signals: Vec<BeliefSignal>`

### 4. ResolveMarket

Resolves a market when belief condition is met.

**Accounts:**

- `[signer]` resolver
- `[writable]` market
- `[]` clock

**Parameters:**

- `inflection: BeliefInflection`

### 5. ClaimPayout

Claims payout from a settled position.

**Accounts:**

- `[signer]` user
- `[writable]` position
- `[writable]` market
- `[writable]` user_token_account
- `[]` token_program

## Belief State Index Calculation

The BSI is calculated on-chain using weighted aggregation:

```
BSI = Σ(signal_i * weight_i) / Σ(weight_i)
```

With temporal decay:

```
weight_effective = weight * decay_factor^(age_seconds / decay_window)
```

## Settlement Curves

### Gaussian Distribution

```
payout = base_amount * 2 * exp(-(distance²) / (2σ²))
```

Where:

- `distance` = time difference from inflection
- `σ` = standard deviation (configurable)

### Linear Decay

```
payout = base_amount * max(0, 2 - distance * decay_rate)
```

## Oracle Integration

### Oracle Update Flow

1. Oracle monitors external data sources
2. Aggregates signals into BeliefSignal format
3. Submits UpdateBSI instruction
4. On-chain program validates and updates BSI
5. Checks for belief condition satisfaction

### Oracle Requirements

- Minimum 3 independent oracle sources
- Maximum 300-second update frequency
- Cryptographic signature verification
- Stake-based reputation system

## Security Considerations

### Access Control

- Market creation: Any user with sufficient SOL
- BSI updates: Authorized oracles only
- Market resolution: Automated based on BSI
- Position withdrawal: Position owner only

### Economic Security

- Minimum position size prevents spam
- Fee mechanism sustains oracle operations
- Stake requirements for oracle participation
- Time-locked withdrawals prevent manipulation

## Gas Optimization

- Compact account structures using Borsh
- Batch oracle updates
- Lazy BSI recalculation
- Efficient PDA derivation

## Upgrade Path

The protocol supports upgrades through:

- Program upgrades via Solana's upgrade authority
- Versioned account structures
- Migration instructions for state transitions

## Integration Guide

### For Developers

```rust
// Initialize client
let client = PredaClient::new(rpc_url, keypair).await?;

// Create market
let market = client.create_market(
    MarketType::SentimentTransition,
    belief_condition,
    description,
).await?;

// Place position
let position = client.place_position(
    &market.address,
    predicted_time,
    amount,
).await?;
```

### For Oracles

```rust
// Submit belief signals
let signals = vec![
    BeliefSignal { /* ... */ },
];

client.update_bsi(&market_address, signals).await?;
```

## Appendix

### Market Type Specifications

Each market type has specific belief condition requirements and resolution logic detailed in the SDK documentation.

### Fee Structure

- Market creation: 0.01 SOL
- Position placement: 0.5% of position size
- Oracle updates: Subsidized by protocol
- Withdrawals: 0.1% of amount

---

For implementation details, see the [Preda SDK documentation](https://docs.rs/preda-sdk).
