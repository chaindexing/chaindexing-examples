## Chaindexing Rust Examples

To run an example:

- Start DB:

```sh
  docker-compose up
```

- Create a `.env` file with the secrets as shown in `.env.sample`

- Run `nfts` example using `cargo run -p nfts` or for `uniswap`, use `cargo run -p uniswap`

## Debugging

Not working? Restart DB per example using:

```sh
  docker-compose down
  docker-compose up
```
