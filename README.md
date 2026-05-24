## Chaindexing Examples

Contains simple examples for indexing smart contracts with Chaindexing across supported
languages. See `README.md` in sub-directories for how to run each example.

### Available Examples

- **`rust/`** - Complete working examples in Rust
  - `nfts/` - Index ERC721 NFT transfers (BAYC, Doodles)
  - `uniswap/` - Index Uniswap V3 pools and swaps
  - `reorg-policies.md` - Realtime, Balanced, FinalityFirst, and outbox finality examples
- **`typescript/`** - TypeScript examples (structure complete, awaiting core implementation)
  - `nfts/` - NFT transfer indexing with TypeScript
  - `uniswap/` - Uniswap V3 protocol indexing with TypeScript
  - Status: ⏳ Ready for when TypeScript core is complete

### Quick Start

Choose your preferred language:

```bash
# Rust examples (fully working)
cd rust/
# See rust/README.md for setup instructions

# TypeScript examples (development in progress)
cd typescript/
npm run test-status  # Check current implementation status
# See typescript/README.md for expected functionality
```

### Reorg and Finality Use Cases

Use Chaindexing's built-in presets instead of tuning reorg internals:

- `Realtime` for dashboards, feeds, and low-latency UI.
- `Balanced` for analytics, reporting, balances, and operational views.
- `FinalityFirst` for payments, claims, settlement, and irreversible workflows.

See [`rust/reorg-policies.md`](rust/reorg-policies.md) for concrete Rust snippets.
