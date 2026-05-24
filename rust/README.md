## Chaindexing Rust Examples

To run an example:

- Start DB:

```sh
  docker-compose up
```

- Create a `.env` file with the secrets as shown in `.env.sample`

- Run `nfts` example using `cargo run -p nfts` or for `uniswap`, use `cargo run -p uniswap`

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
