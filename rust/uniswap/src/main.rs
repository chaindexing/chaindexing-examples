mod event_handlers;
mod states;

use chaindexing::{Chain, ChainId, Contract, Indexer};
use event_handlers::{PoolCreatedEventHandler, SwapEventHandler};
use states::{PoolMigrations, TokenSwapVolumeMigrations};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Setup BAYC's contract
    const UNISWAP_V3_FACTORY_MULTICHAIN_CONTRACT_ADDRESS: &str =
        "0x1F98431c8aD98523631AE4a59f267346ea31F984";
    let uniswap_v3_factory_contract = Contract::<()>::new("UniswapV3Factory")
        .add_event_handler(PoolCreatedEventHandler)
        .add_state_migrations(PoolMigrations)
        .add_address(
            UNISWAP_V3_FACTORY_MULTICHAIN_CONTRACT_ADDRESS,
            &ChainId::Mainnet,
            12369621,
        )
        .add_address(
            UNISWAP_V3_FACTORY_MULTICHAIN_CONTRACT_ADDRESS,
            &ChainId::Arbitrum,
            165,
        )
        .add_address(
            UNISWAP_V3_FACTORY_MULTICHAIN_CONTRACT_ADDRESS,
            &ChainId::Polygon,
            22757547,
        );

    // UniswapV3Pool contract with no addresses yet. They will be populated at runtime.
    let uniswap_v3_pool_contract = Contract::new("UniswapV3Pool")
        .add_event_handler(SwapEventHandler)
        .add_state_migrations(TokenSwapVolumeMigrations);

    // Setup indexer
    let indexer = Indexer::new(&get_database_url())
        .chain(Chain::mainnet(&get_mainnet_json_rpc_url()))
        // Demonstrate slowing it down
        .ingestion_rate_ms(40_000)
        .blocks_per_batch(400)
        .contract(uniswap_v3_factory_contract)
        .contract(uniswap_v3_pool_contract);

    let indexer = if let Ok(arbitrum_json_rpc_url) = std::env::var("ARBITRUM_JSON_RPC_URL") {
        indexer.chain(Chain::arbitrum(&arbitrum_json_rpc_url))
    } else {
        indexer
    };

    let indexer = if let Ok(polygon_json_rpc_url) = std::env::var("POLYGON_JSON_RPC_URL") {
        indexer.chain(Chain::polygon(&polygon_json_rpc_url))
    } else {
        indexer
    };

    println!("Chaindexing is taking a moment to setup...");
    indexer.run().await.unwrap();
    println!("Chaindexing is indexing UniswapV3Factory contract...");
    println!("Query via 'SELECT * from uniswap_pools' to see pools per chain.");
    println!("Also query `uniswap_token_swap_volumes` to view total swap volume per token");

    loop {
        // Infinite loop to keep the main thread running
        std::thread::sleep(std::time::Duration::from_millis(500))
    }
}

fn get_database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn get_mainnet_json_rpc_url() -> String {
    std::env::var("MAINNET_JSON_RPC_URL").expect("MAINNET_JSON_RPC_URL must be set")
}
