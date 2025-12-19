# Preda SDK

<div align="center">

**Rust SDK for Time-Shifted Prediction Markets on Solana**


[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

</div>

## 🎯 Overview

**Preda SDK** is a comprehensive Rust library for interacting with the Preda protocol - a decentralized forecasting system that predicts **when collective belief will change** rather than what will happen.

Unlike traditional prediction markets that resolve on discrete outcomes, Preda enables markets that resolve based on measurable inflection points in consensus, sentiment, or probabilistic belief.

## 📦 What We've Built

### Core Components

#### 1. **Type System** (`src/types/`)

- **Belief Types** - BeliefStateIndex, BeliefCondition, BeliefInflection, BeliefSignal
- **Market Types** - Market, MarketType, MarketState, MarketConfig, SettlementCurve
- **Position Types** - Position, TimeBucket, PositionStatus, TimeBucketAggregate

#### 2. **Belief State Index (BSI)** (`src/bsi/`)

- **Calculator** - Sophisticated BSI computation with:
  - Weighted signal aggregation
  - Outlier filtering using z-scores
  - Velocity and volatility calculation
  - Confidence scoring
  - Temporal decay functions
- **Aggregator** - Signal buffering and analysis:
  - Multi-source signal aggregation
  - Statistical analysis (mean, median, std dev)
  - Time-windowed queries
  - Source diversity metrics
- **Monitor** - Real-time inflection detection:
  - Sentiment reversal detection
  - Threshold crossing detection
  - Velocity spike detection
  - Persistence validation
  - Callback system for events

#### 3. **Oracle Integration** (`src/oracle/`)

- **Sentiment Oracle** - Social sentiment tracking
- **Narrative Oracle** - Narrative framing and topic dominance
- **Forecast Oracle** - Probabilistic forecast aggregation
- **Consensus Oracle** - AI model agreement measurement

#### 4. **Market Operations** (`src/market/`)

- **Market Manager** - Create and manage markets
- **Lifecycle Manager** - State transitions
- **Settlement Calculator** - Volatility-aware payouts with multiple curves:
  - Linear decay
  - Exponential decay
  - Gaussian distribution
  - Custom curves

#### 5. **Client** (`src/client.rs`)

- Unified interface for all operations
- Async/await support
- Type-safe API
- Comprehensive error handling

## 🚀 Key Features

### 1. **Time-Shifted Prediction**

Markets resolve based on **when** belief changes, not **what** happens:

- Sentiment transition markets
- Probability threshold markets
- Model consensus markets
- Narrative velocity markets

### 2. **Belief State Index (BSI)**

Continuously updated aggregation of belief signals:

- Multi-source data integration
- Temporal smoothing and decay
- Noise reduction and outlier filtering
- Confidence scoring

### 3. **Oracle Framework**

Modular oracle system for belief measurement:

- Social sentiment oracles
- News and narrative oracles
- Forecast aggregation oracles
- AI consensus oracles

### 4. **Volatility-Aware Settlement**

Sophisticated payout mechanisms:

- Multiple settlement curves
- Sharpness-adjusted payouts
- Distance-based rewards
- Pool distribution normalization

### 5. **Solana-Optimized**

Built for high-performance blockchain:

- Low-latency belief updates
- Parallel market execution
- Efficient account structures
- Gas-optimized operations

## 📊 Market Types

### 1. Sentiment Transition Markets

```rust
BeliefCondition::SentimentShift {
    from_polarity: -0.2,
    to_polarity: 0.6,
    persistence_window: 3600,
}
```

### 2. Probability Threshold Markets

```rust
BeliefCondition::ProbabilityThreshold {
    threshold: 0.6,
    direction: ThresholdDirection::Above,
    persistence_window: 7200,
}
```

### 3. Model Consensus Markets

```rust
BeliefCondition::ModelConsensus {
    min_models: 5,
    convergence_band: 0.1,
    persistence_window: 3600,
}
```

### 4. Narrative Velocity Markets

```rust
BeliefCondition::NarrativeVelocity {
    velocity_threshold: 0.5,
    acceleration_threshold: 0.2,
    persistence_window: 1800,
}
```

## 💡 Usage Examples

### Creating a Market

```rust
let client = PredaClient::new(rpc_url, keypair).await?;

let market = client.create_market(
    MarketType::SentimentTransition,
    BeliefCondition::SentimentShift {
        from_polarity: -0.2,
        to_polarity: 0.6,
        persistence_window: 3600,
    },
    "BTC sentiment turns bullish",
).await?;
```

### Placing a Position

```rust
let position = client.place_position(
    &market.address,
    predicted_timestamp,
    1_000_000_000, // 1 SOL
).await?;
```

### Monitoring BSI

```rust
let monitor = BeliefMonitor::new(0.5, 300);

monitor.on_inflection(|inflection| {
    println!("Inflection detected: {:?}", inflection);
}).await;

let bsi = client.get_belief_state_index(&market_address).await?;
monitor.update(bsi).await?;
```

## 🔧 Installation & Setup

Add this to your `Cargo.toml`:

```toml
[dependencies]
preda-sdk = "0.1.0"
```

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

## 🏗️ Project Structure

```text
preda-sdk/
├── Cargo.toml                 # Project manifest
├── README.md                  # Main documentation
├── LICENSE-MIT                # MIT License
├── LICENSE-APACHE             # Apache License
├── .gitignore                 # Git ignore rules
├── .github/
│   └── workflows/
│       └── ci.yml             # CI/CD pipeline
├── src/
│   ├── lib.rs                 # Library entry point
│   ├── client.rs              # Main client
│   ├── error.rs               # Error types
│   ├── types/                 # Core types
│   │   ├── mod.rs
│   │   ├── belief.rs
│   │   ├── market.rs
│   │   └── position.rs
│   ├── bsi/                   # Belief State Index
│   │   ├── mod.rs
│   │   ├── calculator.rs
│   │   ├── aggregator.rs
│   │   └── monitor.rs
│   ├── oracle/                # Oracle integrations
│   │   ├── mod.rs
│   │   ├── sentiment.rs
│   │   ├── narrative.rs
│   │   ├── forecast.rs
│   │   └── consensus.rs
│   └── market/                # Market operations
│       ├── mod.rs
│       ├── lifecycle.rs
│       └── settlement.rs
├── examples/                  # Usage examples
│   ├── create_market.rs
│   ├── place_position.rs
│   └── query_bsi.rs
└── docs/
    └── protocol.md            # Protocol specification
```

## 🔬 Technical Highlights

### Error Handling

Comprehensive error types with context:

- Solana client errors
- Oracle errors
- BSI calculation errors
- Market state errors
- Position errors

### Async Architecture

Built on Tokio for efficient concurrency:

- Non-blocking I/O
- Parallel oracle queries
- Concurrent market monitoring
- Efficient resource usage

### Type Safety

Strongly typed Rust API:

- Compile-time guarantees
- No runtime type errors
- Clear interfaces
- Self-documenting code

### Testing

Comprehensive test coverage:

- Unit tests for all modules
- Integration tests for workflows
- Property-based tests
- Mock oracle implementations

## 🛠️ Development Tools

### CI/CD Pipeline

GitHub Actions workflow:

- Automated testing
- Clippy linting
- Format checking
- Release builds

### Code Quality

- Rustfmt for formatting
- Clippy for linting
- Comprehensive tests
- Documentation coverage

## 🌟 Innovation Points

### 1. **Temporal Prediction Primitive**

First protocol to make **timing of belief change** a first-class object

### 2. **Reflexivity Observation**

Built-in tools to observe feedback loops between belief, pricing, and information

### 3. **Continuous Resolution**

Smooth, volatility-aware settlement vs. binary outcomes

### 4. **Multi-Oracle Architecture**

Aggregates diverse belief signals for robust measurement

### 5. **Web3-Native Design**

Fully decentralized, on-chain execution on Solana

## 🎯 Use Cases

1. **Financial Markets** - Anticipate sentiment reversals
2. **Policy Analysis** - Track consensus formation
3. **Social Dynamics** - Detect narrative shifts
4. **AI Research** - Compare human vs. model beliefs
5. **Risk Management** - Measure expectation dynamics

## 🤝 Contributing to Preda SDK

Thank you for your interest in contributing to Preda SDK!

### Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

### How to Contribute

#### Reporting Bugs

- Check if the bug has already been reported in Issues
- Use the bug report template
- Include detailed steps to reproduce
- Provide system information and SDK version

#### Suggesting Features

- Check if the feature has been suggested
- Clearly describe the use case
- Explain how it aligns with Preda's vision

#### Pull Requests

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes**:
   - Write clean, documented code
   - Follow Rust conventions
   - Add tests for new functionality
   - Update documentation
4. **Run tests**: `cargo test`
5. **Run clippy**: `cargo clippy -- -D warnings`
6. **Format code**: `cargo fmt`
7. **Commit changes**: Use clear, descriptive commit messages
8. **Push to your fork**: `git push origin feature/your-feature-name`
9. **Open a Pull Request**

### Code Style

- Follow Rust naming conventions
- Use meaningful variable and function names
- Add doc comments for public APIs
- Keep functions focused and concise
- Write tests for new functionality

### Development Setup

```bash
# Clone the repository
git clone https://github.com/predaprotocol/preda-sdk.git
cd preda-sdk

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example create_market
```

### Testing Guidelines

- Write unit tests for individual functions
- Write integration tests for workflows
- Ensure all tests pass before submitting PR
- Aim for high code coverage

### Documentation Requirements

- Update README.md if adding features
- Add inline documentation for public APIs
- Update examples if changing interfaces
- Keep documentation clear and concise

### Commit Message Format

Format: `<type>: <description>`

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding tests
- `refactor`: Code refactoring
- `chore`: Maintenance tasks

Example: `feat: add narrative velocity market type`

### Review Process

1. Maintainers will review your PR
2. Address feedback and requested changes
3. Once approved, your PR will be merged

### Questions?

- Open an issue for questions
- Join our Discord community
- Check existing documentation

## 🚀 Next Steps

### For Users

1. Review examples in `examples/`
2. Read protocol specification in `docs/protocol.md`
3. Experiment with test markets
4. Join community Discord

### For Contributors

1. Read this Contributing section
2. Check open issues on GitHub
3. Submit PRs for improvements
4. Help improve documentation

### For Integrators

1. Study API documentation at [docs.rs/preda-sdk](https://docs.rs/preda-sdk)
2. Implement custom oracle adapters
3. Build frontend interfaces
4. Create market strategies

## 📈 Roadmap

- [x] Phase 1: Core SDK with BSI and basic market operations
- [ ] Phase 2: Advanced oracle integrations
- [ ] Phase 3: Volatility-aware settlement
- [ ] Phase 4: Reflexivity analysis tools
- [ ] Phase 5: Cross-chain support

## 📄 License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

## 🌐 Community

- **Website**: [predaprotocol.com](https://predaprotocol.com/)
- **Documentation**: [GitBook](https://predaprotocol.gitbook.io/documentation/)
- **Twitter**: [@predaprotocol](https://x.com/predaprotocol)

## 🙏 Acknowledgments

Built with ❤️ by the Preda team and contributors.

> **"Predict not the event — predict when consensus will flip."**
