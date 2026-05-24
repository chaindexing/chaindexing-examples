# Reorg Policy Examples

These examples show how to choose Chaindexing finality behavior by product use case.
The reorg algorithm stays the same internally: canonical block tracking, block-hash log
reads, scan records, and deterministic replay.

## Realtime Dashboard

Use this for activity feeds, portfolio views, and UI where low latency matters and
occasional replay is acceptable.

```rust
use chaindexing::{Chain, Contract, Indexer, ReorgMode};

Indexer::new(&database_url)
    .chain(Chain::mainnet(&mainnet_rpc_url))
    .contract(nft_contract)
    .reorg_mode(ReorgMode::Realtime)
    .run()
    .await?;
```

## Balanced Analytics

Use this for analytics, reporting, leaderboards, and operational views that should avoid
most near-head churn.

```rust
use chaindexing::{Chain, Contract, Indexer, ReorgMode};

Indexer::new(&database_url)
    .chain(Chain::mainnet(&mainnet_rpc_url))
    .contract(uniswap_contract)
    .reorg_mode(ReorgMode::Balanced)
    .run()
    .await?;
```

## Payments, Claims, and Settlement

Use finality-first indexing when an indexed event can trigger money movement,
settlement, claims, or any hard-to-reverse workflow.

```rust
use chaindexing::{Chain, Contract, Indexer, ReorgMode};

Indexer::new(&database_url)
    .chain(Chain::mainnet(&mainnet_rpc_url))
    .contract(claims_contract)
    .reorg_mode(ReorgMode::FinalityFirst)
    .run()
    .await?;
```

## Custom Confirmation Depth

Use explicit confirmations for chains or RPC providers where `safe`/`finalized` are not
available or not appropriate.

```rust
use chaindexing::{Chain, Contract, Indexer, IndexingFinality, SideEffectFinality};

Indexer::new(&database_url)
    .chain(Chain::mainnet(&mainnet_rpc_url))
    .contract(nft_contract)
    .indexing_finality(IndexingFinality::LatestWithConfirmations(12))
    .side_effect_finality(SideEffectFinality::Confirmations(24))
    .run()
    .await?;
```

## Outbox Webhook After Safe/Finalized Blocks

Index quickly, but only dispatch customer-visible webhooks after a finality watermark.

```rust
use chaindexing::{
    dispatch_pending_outbox_jobs, OutboxDispatchConfig, OutboxFinalityWatermark,
    SideEffectFinality,
};

context
    .enqueue_outbox_with_configured_finality("customer-webhook", &payload)
    .await;

// Or override a single job:
context
    .enqueue_outbox_with_finality("settlement-webhook", &payload, SideEffectFinality::Finalized)
    .await;

let dispatch_config = OutboxDispatchConfig::default().with_finality_watermark(
    OutboxFinalityWatermark {
        chain_id: 1,
        latest_block_number: Some(latest),
        safe_block_number: Some(safe),
        finalized_block_number: Some(finalized),
    },
);

dispatch_pending_outbox_jobs(&database_url, &dispatcher, dispatch_config).await;
```

Pending outbox rows sourced from reorged events are moved to `cancelled_reorg` before
new leases are issued.
