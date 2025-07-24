# Chaindexing TypeScript Examples

TypeScript examples demonstrating how to index smart contracts using Chaindexing.

> ⚠️ **Note**: The TypeScript implementation of Chaindexing is currently in development. These examples show the intended API and structure once the TypeScript version is complete and matches the functionality of the Rust version.

## Current Status

The examples are structurally complete and follow the same patterns as the Rust examples, but cannot run yet due to:

1. **Incomplete TypeScript Core**: The chaindexing-ts implementation is still being developed
2. **Import Structure Issues**: The TypeScript packages need proper build/publish setup
3. **Missing Features**: Some functionality like `readOne` for state queries is not implemented yet

## Examples Structure

The examples are designed to work exactly like the Rust versions:

### NFTs Example (`/nfts`) ✅ Structure Complete

- **`states.ts`**: NFT state class and database migrations
- **`event-handlers.ts`**: Transfer event processing logic
- **`main.ts`**: Application setup and configuration
- **`README.md`**: Documentation and usage instructions

### Uniswap Example (`/uniswap`) ✅ Structure Complete

- **`states.ts`**: Pool and TokenSwapVolume state classes
- **`event-handlers.ts`**: Pool creation and swap event logic
- **`main.ts`**: Multi-chain configuration setup
- **`README.md`**: Documentation and usage instructions

## Prerequisites

- Node.js >= 18.0.0
- PostgreSQL database
- Blockchain RPC endpoints (Alchemy, Infura, etc.)

## Setup (Ready for when TS implementation is complete)

1. **Start Database:**

   ```bash
   docker-compose up -d
   ```

2. **Install Dependencies:**

   ```bash
   npm install
   ```

3. **Configure Environment:**

   ```bash
   cp .env.sample .env
   # Edit .env with your database connection and RPC URLs
   ```

4. **Run Examples:** (Once TypeScript implementation is ready)
   ```bash
   npm run nfts      # NFTs example
   npm run uniswap   # Uniswap example
   ```

## Development Progress

- ✅ **Project Structure**: Complete with proper TypeScript configuration
- ✅ **Example Code**: NFTs and Uniswap examples following Rust patterns
- ✅ **Documentation**: READMEs and code comments
- ✅ **Database Setup**: Docker Compose and initialization scripts
- ⏳ **Core Implementation**: Waiting for chaindexing-ts completion
- ⏳ **Import Resolution**: Needs proper TypeScript package structure
- ⏳ **Feature Parity**: Some methods like state reading not implemented

## Key Features (When Complete)

- **Multi-chain support** - Index multiple EVM chains simultaneously
- **Type-safe** - Full TypeScript support with comprehensive types
- **Same API as Rust** - Consistent experience across languages
- **Event handlers** - Pure handlers for indexing, side-effect handlers for notifications
- **State management** - Contract, chain, and multi-chain states
- **SQL queries** - Query indexed data using standard SQL

## Architecture

Each example follows the established chaindexing pattern:

```typescript
// State Definition
class MyState extends BaseContractState {
  constructor(
    public id: number,
    public value: string
  ) {
    super();
  }
  tableName(): string {
    return 'my_states';
  }
}

// Event Handler
class MyHandler implements PureHandler {
  abi(): string {
    return 'event MyEvent(uint256 id, string value)';
  }

  async handleEvent(context: PureHandlerContext): Promise<void> {
    const params = context.getEventParams();
    const state = new MyState(params.getU32('id'), params.getString('value'));
    await state.create(context);
  }
}

// Configuration
const config = new Config(new PostgresRepo(DATABASE_URL))
  .addChain(Chain.Mainnet, RPC_URL)
  .addContract(
    createContract('MyContract')
      .addAddress('0x...', Chain.Mainnet, START_BLOCK)
      .addEventHandler(new MyHandler())
      .addStateMigrations(new MyMigrations())
      .build()
  );

await indexStates(config);
```

## Next Steps

1. **Complete chaindexing-ts core implementation**
2. **Set up proper TypeScript package publishing**
3. **Implement missing features like state reading**
4. **Add comprehensive testing**
5. **Update examples as TypeScript implementation progresses**

## Contributing

These examples serve as a specification for what the TypeScript implementation should support. They can be used to guide development and testing of the core TypeScript chaindexing library.

See individual example READMEs for detailed documentation on expected functionality.
