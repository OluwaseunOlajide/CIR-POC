# Blockchain: MEV-Resistant Oracles

## The Problem

Decentralized exchanges (DEXs) use AI oracles to detect arbitrage opportunities and price discrepancies across chains. When oracle computation time varies based on opportunity size, **MEV (Maximal Extractable Value) bots exploit timing patterns to front-run trades.**

**Without CIR:**
- Stable market query: 6ms → "No arbitrage opportunity"
- Large arbitrage detected: 180ms → "$2.3M arbitrage opportunity found"

**MEV bots monitoring oracle timing can detect profitable opportunities and front-run legitimate traders, extracting value unfairly.**

This undermines **fair execution** and creates an unfair advantage for sophisticated attackers.

<img src=https://github.com/OluwaseunOlajide/CIR-POC/blob/main/use-cases/blockchain/blockchain-use-case.png>

## The Solution

CIR ensures constant-time execution regardless of arbitrage opportunity size:

**With CIR:**
- Stable market query: 90ms → "No arbitrage opportunity"
- Large arbitrage detected: 90ms → "$2.3M arbitrage opportunity"

**Timing reveals nothing about opportunity size.** All oracle queries take identical time.

## Implementation
```python
from cir import ConstantTimeRuntime

# Initialize runtime for MEV-resistant oracle
runtime = ConstantTimeRuntime(
    backend='sev-snp',
    attestation=True
)

# Load arbitrage detection model
model = load_model('cross_chain_arbitrage_v4')

# Query oracle (constant-time execution)
result = runtime.infer(model, {
    'eth_prices': get_eth_prices_all_chains(),
    'liquidity_depths': get_liquidity_data(),
    'gas_costs': estimate_gas_costs()
})

# Get attestation proof
proof = runtime.get_attestation()

# Publish result with attestation
publish_oracle_result({
    'arbitrage_opportunity': result.opportunity_exists,
    'opportunity_size': result.estimated_profit_usd,
    'attestation': proof.signature,
    'execution_time': proof.constant_time_ms,  # Same for all queries
    'timestamp': block.timestamp
})
```

## Results

- **Overhead:** 4.1% average
- **Timing variance:** <1ms (constant-time guaranteed)
- **MEV resistance:** ✅ Opportunity size hidden from bots
- **Fairness:** Level playing field for all traders

## Who Needs This

**DeFi protocols:**
- Decentralized exchanges (DEXs)
- Cross-chain bridges
- Lending protocols
- Derivatives platforms

**Oracle providers:**
- Price oracles (Chainlink, etc.)
- Volatility oracles
- Cross-chain data feeds
- AI-powered prediction markets

**Threat scenarios:**
- MEV bots extracting value via timing
- Sandwich attacks based on timing patterns
- Front-running based on computation time
- Unfair advantage for sophisticated actors

## Technical Details

**Threat model:**
- MEV bots monitor oracle execution timing
- Timing reveals arbitrage opportunity size
- Goal: Front-run trades for profit extraction

**CIR protection:**
- All oracle queries take identical time
- Large and small opportunities indistinguishable
- Cryptographic proof of fair execution

**On-chain verification:**
- Hardware attestation published on-chain
- Users verify constant-time execution
- Transparent fairness guarantees

**Integration:**
- Compatible with EVM chains
- Works with Cosmos SDK
- Supports multiple TEE backends

---

[← Back to all use cases](../)
