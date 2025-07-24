# NFTs Example

This example demonstrates indexing ERC721 NFT Transfer events using Chaindexing TypeScript.

## What it does

- Monitors **BAYC (Bored Ape Yacht Club)** and **Doodles** NFT contracts
- Indexes all `Transfer` events (mints and transfers)
- Stores NFT ownership data in PostgreSQL
- Demonstrates both pure handlers (for deterministic indexing) and side-effect handlers (for notifications)

## Contracts Monitored

- **BAYC**: `0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D` (from block 17773490)
- **Doodles**: `0x8a90CAb2b38dba80c64b7734e58Ee1dB38B8992e` (from block 17769635)

## Running

1. **Set up environment:**
   ```bash
   cp ../.env.sample ../.env
   # Edit .env with your database URL and Ethereum RPC endpoint
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
   npm run nfts
   ```

## Database Schema

The example creates an `nfts` table:

```sql
CREATE TABLE IF NOT EXISTS nfts (
  token_id INTEGER NOT NULL,
  owner_address TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);
```

## Querying Data

Connect to your PostgreSQL database and run:

```sql
-- View all indexed NFTs
SELECT * FROM nfts;

-- Count NFTs by owner
SELECT owner_address, COUNT(*) as nft_count 
FROM nfts 
GROUP BY owner_address 
ORDER BY nft_count DESC;

-- Recent transfers
SELECT * FROM nfts 
ORDER BY created_at DESC 
LIMIT 10;
```

## Architecture

- **`states.ts`**: Defines the `Nft` state class and database migrations
- **`event-handlers.ts`**: Contains the event processing logic
  - `TransferHandler`: Processes Transfer events and updates database
  - `TransferSideEffectHandler`: Sends notifications (mock implementation)
- **`main.ts`**: Application entry point and configuration 