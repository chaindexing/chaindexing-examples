# Uniswap Example

This example demonstrates indexing Uniswap V3 protocol events using Chaindexing TypeScript.

## What it does

- Monitors the **Uniswap V3 Factory** contract for `PoolCreated` events
- Tracks all new liquidity pools as they're created
- Monitors **Uniswap V3 Pool** contracts for `Swap` events
- Calculates token swap volumes across all monitored pools
- Supports multiple chains (Ethereum, Arbitrum, Polygon)

## Contracts Monitored

- **Uniswap V3 Factory**: `0x1F98431c8aD98523631AE4a59f267346ea31F984` (from block 12369621)
- **Uniswap V3 Pools**: Dynamic - automatically added when pools are created

## Running

1. **Set up environment:**
   ```bash
   cp ../.env.sample ../.env
   # Edit .env with your database URL and RPC endpoints
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Start the database:**
   ```bash
   cd ../.. && docker-compose up -d
   ```

4. **Run the example:**
   ```bash
   npm run uniswap
   ```

## Database Schema

The example creates two tables:

### `uniswap_pools`
```sql
CREATE TABLE IF NOT EXISTS uniswap_pools (
  token0_address VARCHAR NOT NULL,
  token1_address VARCHAR NOT NULL,
  pool_contract_address VARCHAR NOT NULL,
  fee INTEGER NOT NULL,
  tick_spacing INTEGER NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);
```

### `uniswap_token_swap_volumes`
```sql
CREATE TABLE IF NOT EXISTS uniswap_token_swap_volumes (
  token_address VARCHAR NOT NULL PRIMARY KEY,
  amount_ether VARCHAR NOT NULL,
  last_updated_at BIGINT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);
```

## Querying Data

Connect to your PostgreSQL database and run:

```sql
-- View all indexed pools
SELECT * FROM uniswap_pools ORDER BY created_at DESC;

-- Count pools by fee tier
SELECT fee, COUNT(*) as pool_count 
FROM uniswap_pools 
GROUP BY fee 
ORDER BY fee;

-- View token swap volumes
SELECT * FROM uniswap_token_swap_volumes 
ORDER BY CAST(amount_ether AS DECIMAL) DESC 
LIMIT 10;

-- Recent pool creations
SELECT 
  token0_address,
  token1_address,
  pool_contract_address,
  fee,
  created_at
FROM uniswap_pools 
ORDER BY created_at DESC 
LIMIT 5;
```

## Multi-Chain Support

The example supports multiple chains by configuring additional RPC endpoints:

```bash
# In your .env file
MAINNET_JSON_RPC_URL=https://eth-mainnet.g.alchemy.com/v2/your-api-key
ARBITRUM_JSON_RPC_URL=https://arb-mainnet.g.alchemy.com/v2/your-api-key
POLYGON_JSON_RPC_URL=https://polygon-mainnet.g.alchemy.com/v2/your-api-key
```

## Architecture

- **`states.ts`**: Defines the `Pool` and `TokenSwapVolume` state classes with database migrations
- **`event-handlers.ts`**: Contains the event processing logic
  - `PoolCreatedEventHandler`: Processes new pool creation events
  - `SwapEventHandler`: Processes swap transactions and updates volume metrics
- **`main.ts`**: Application entry point and multi-chain configuration

## Performance

The example demonstrates rate limiting similar to the Rust version:
- **Ingestion rate**: 40 seconds between batches
- **Blocks per batch**: 400 blocks
- **Dynamic contract inclusion**: New pools are automatically added for indexing

This slower pace helps demonstrate the indexing process and reduces API rate limit issues during development. 