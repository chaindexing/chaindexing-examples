## Chaindexing Rust Examples

To run an example:

- Start DB:

```sh
  docker-compose up
```

- Create a `.env` file with the secrets as shown in `.env.sample`

- Run `nfts` example using `cargo run -p nfts` or for `uniswap`, use `cargo run -p uniswap`

## Runtime profile examples

The examples use workload-oriented runtime profiles instead of environment names:

- Runtime profile examples require `chaindexing >= 0.1.81`.
- `nfts` uses `RuntimeConfig::realtime().blocks_per_batch(400)` for app-feed style indexing.
- `uniswap` uses `RuntimeConfig::catchup_then_realtime()` with explicit DB, worker, and RPC limits for a multi-chain workload with runtime-discovered pool contracts.

Other common shapes:

```rust
// Historical catch-up.
RuntimeConfig::backfill()
    .limits(RuntimeLimits::throughput())
    .rpc(RpcPolicy::throughput().max_in_flight(64).max_per_chain(16));

// Cheap or rate-limited RPC providers.
RuntimeConfig::rpc_constrained()
    .rpc(RpcPolicy::limited().max_in_flight(4).max_per_chain(2).requests_per_second(5));

// Reproducible local or CI runs.
RuntimeConfig::deterministic();
```

## Reorg and finality examples

Use [`reorg-policies.md`](reorg-policies.md) to choose a preset for common product
classes:

- Realtime dashboards and activity feeds.
- Balanced analytics and reporting.
- Finality-first payments, claims, and settlement.
- Outbox webhooks that index quickly but dispatch after safe/finalized blocks.

## Debugging

Not working? Restart DB per example using:

```sh
  docker-compose down
  docker-compose up
```
