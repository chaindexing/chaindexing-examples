import { indexStates } from '../../../../chaindexing-ts/chaindexing/src';
import { Chain, createContract } from '../../../../chaindexing-ts/chaindexing-core/src';
import { Config } from '../../../../chaindexing-ts/chaindexing-config/src';
import { PostgresRepo } from '../../../../chaindexing-ts/chaindexing-postgres/src';
import { TransferHandler, TransferSideEffectHandler } from './event-handlers';
import { NftMigrations } from './states';
import * as dotenv from 'dotenv';

// Load environment variables
dotenv.config();

interface AppState {
  notificationCount: number;
  lastProcessedBlock: number;
}

async function main() {
  try {
    console.log('🚀 Starting NFTs Example - Chaindexing TypeScript');

    // Create the NFTs contract configuration
    const nftsContract = createContract<AppState>('ERC721')
      // Add transfer event and its corresponding handlers
      .addEventHandler(new TransferHandler())
      .addSideEffectHandler(new TransferSideEffectHandler())
      // Add migration for the state's DB schema
      .addStateMigrations(new NftMigrations())
      // Add contract address for BAYC (Bored Ape Yacht Club)
      .addAddress(
        '0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D',
        Chain.Mainnet,
        17773490
      )
      // Add contract address for Doodles
      .addAddress(
        '0x8a90CAb2b38dba80c64b7734e58Ee1dB38B8992e',
        Chain.Mainnet,
        17769635
      )
      .build();

    // Setup indexing configuration
    const config = new Config(
      // Database
      new PostgresRepo(getDatabaseUrl())
    )
      .withBlocksPerBatch(400)
      // Add all possible chains in your Dapp
      .addChain(Chain.Mainnet, getMainnetJsonRpcUrl())
      // Add BAYC's and Doodles' contracts
      .addContract(nftsContract)
      // Add shared state for side effects
      .withInitialState<AppState>({
        notificationCount: 0,
        lastProcessedBlock: 0,
      });

    console.log('📊 Configuration:');
    console.log(`   - Database: ${getDatabaseUrl().replace(/\/\/.*@/, '//***@')}`);
    console.log(`   - Chains: ${Array.from(config.chains.keys()).join(', ')}`);
    console.log(`   - Contracts: ${config.contracts.map((c) => c.name).join(', ')}`);
    console.log(`   - Blocks per batch: ${config.blocksPerBatch}`);

    console.log('⏳ Chaindexing is taking a moment to setup...');

    // Start Indexing Process
    await indexStates(config);

    console.log('✅ Chaindexing is indexing states for BAYC and Doodles contracts...');
    console.log('🔍 Query "nfts" table using "SELECT * from nfts" to see populated indices.');

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
    console.error('❌ Error starting NFTs example:', error);
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