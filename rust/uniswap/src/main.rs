mod event_handlers;
mod states;

use chaindexing::{Chain, ChainId, Config, Contract, PostgresRepo};
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

    // Setup config
    let config = Config::new(PostgresRepo::new(&get_database_url()))
        .add_chain(Chain::new(ChainId::Mainnet, &get_mainnet_json_rpc_url()))
        // Demonstrate slowing it down
        .with_ingestion_rate_ms(40_000)
        .with_blocks_per_batch(2_000)
        .add_contract(uniswap_v3_factory_contract)
        .add_contract(uniswap_v3_pool_contract);

    let config = if let Ok(arbitrum_json_rpc_url) = std::env::var("ARBITRUM_JSON_RPC_URL") {
        config.add_chain(Chain::new(ChainId::Arbitrum, &arbitrum_json_rpc_url))
    } else {
        config
    };

    let config = if let Ok(polygon_json_rpc_url) = std::env::var("POLYGON_JSON_RPC_URL") {
        config.add_chain(Chain::new(ChainId::Polygon, &polygon_json_rpc_url))
    } else {
        config
    };

    println!("Chaindexing is taking a moment to setup...");
    // Start Indexing Process
    chaindexing::index_states(&config).await.unwrap();
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
