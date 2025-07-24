import { indexStates } from '../../../../chaindexing-ts/chaindexing/src';
import { Chain, createContract } from '../../../../chaindexing-ts/chaindexing-core/src';
import { Config } from '../../../../chaindexing-ts/chaindexing-config/src';
import { PostgresRepo } from '../../../../chaindexing-ts/chaindexing-postgres/src';
import { PoolCreatedEventHandler, SwapEventHandler } from './event-handlers';
import { PoolMigrations, TokenSwapVolumeMigrations } from './states';
import * as dotenv from 'dotenv';

// Load environment variables
dotenv.config();

interface AppState {
  totalPoolsCreated: number;
  totalSwapsProcessed: number;
  lastProcessedBlock: number;
}

async function main() {
  try {
    console.log('🚀 Starting Uniswap Example - Chaindexing TypeScript');

    // Uniswap V3 Factory contract address (same across all chains)
    const UNISWAP_V3_FACTORY_ADDRESS = '0x1F98431c8aD98523631AE4a59f267346ea31F984';

    // Setup Uniswap V3 Factory contract
    const uniswapV3FactoryContract = createContract<AppState>('UniswapV3Factory')
      .addEventHandler(new PoolCreatedEventHandler())
      .addStateMigrations(new PoolMigrations())
      .addAddress(UNISWAP_V3_FACTORY_ADDRESS, Chain.Mainnet, 12369621)
      // Add other chains if RPC URLs are provided
      .build();

    // UniswapV3Pool contract with no addresses yet (they will be populated at runtime by PoolCreated events)
    const uniswapV3PoolContract = createContract<AppState>('UniswapV3Pool')
      .addEventHandler(new SwapEventHandler())
      .addStateMigrations(new TokenSwapVolumeMigrations())
      .build();

    // Setup indexing configuration  
    let config = new Config(new PostgresRepo(getDatabaseUrl()))
      .addChain(Chain.Mainnet, getMainnetJsonRpcUrl())
      // Demonstrate slowing it down (like the Rust example)
      .withIngestionRateMs(40000)
      .withBlocksPerBatch(400)
      .addContract(uniswapV3FactoryContract)
      .addContract(uniswapV3PoolContract)
      .withInitialState<AppState>({
        totalPoolsCreated: 0,
        totalSwapsProcessed: 0,
        lastProcessedBlock: 0,
      });

    // Add Arbitrum chain if RPC URL is provided
    const arbitrumJsonRpcUrl = process.env.ARBITRUM_JSON_RPC_URL;
    if (arbitrumJsonRpcUrl) {
      console.log('📡 Adding Arbitrum chain...');
      // Note: In the real implementation, we'd need to add Chain.Arbitrum
      // For now, we'll just add to the factory contract
      // config = config.addChain(Chain.Arbitrum, arbitrumJsonRpcUrl);
    }

    // Add Polygon chain if RPC URL is provided
    const polygonJsonRpcUrl = process.env.POLYGON_JSON_RPC_URL;
    if (polygonJsonRpcUrl) {
      console.log('📡 Adding Polygon chain...');
      // Note: In the real implementation, we'd need to add Chain.Polygon
      // For now, we'll just add to the factory contract
      // config = config.addChain(Chain.Polygon, polygonJsonRpcUrl);
    }

    console.log('📊 Configuration:');
    console.log(`   - Database: ${getDatabaseUrl().replace(/\/\/.*@/, '//***@')}`);
    console.log(`   - Chains: ${Array.from(config.chains.keys()).join(', ')}`);
    console.log(`   - Contracts: ${config.contracts.map((c) => c.name).join(', ')}`);
    console.log(`   - Blocks per batch: ${config.blocksPerBatch}`);
    console.log(`   - Ingestion rate: ${config.ingestionRateMs}ms`);

    console.log('⏳ Chaindexing is taking a moment to setup...');

    // Start Indexing Process
    await indexStates(config);

    console.log('✅ Chaindexing is indexing UniswapV3Factory contract...');
    console.log('🔍 Query via "SELECT * from uniswap_pools" to see pools per chain.');
    console.log('📊 Also query "uniswap_token_swap_volumes" to view total swap volume per token');

    // Keep running
    process.on('SIGINT', () => {
      console.log('\n👋 Shutting down gracefully...');
      process.exit(0);
    });

    // Infinite loop to keep the main thread running
    while (true) {
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  } catch (error) {
    console.error('❌ Error starting Uniswap example:', error);
    process.exit(1);
  }
}

function getDatabaseUrl(): string {
  const url = process.env.DATABASE_URL;
  if (!url) {
    throw new Error('DATABASE_URL environment variable must be set');
  }
  return url;
}

function getMainnetJsonRpcUrl(): string {
  const url = process.env.MAINNET_JSON_RPC_URL;
  if (!url) {
    throw new Error('MAINNET_JSON_RPC_URL environment variable must be set');
  }
  return url;
}

// Run the example
if (require.main === module) {
  main().catch(console.error);
} 