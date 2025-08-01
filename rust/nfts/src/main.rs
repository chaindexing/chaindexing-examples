mod event_handlers;
mod states;

use chaindexing::{Chain, ChainId, Config, Contract, PostgresRepo};
use states::NftMigrations;

use event_handlers::{TransferHandler, TransferSideEffectHandler};

#[tokio::main]
async fn main() {
    // Setup nft's contract
    let nfts_contract = Contract::new("ERC721")
        // add transfer event and its corresponding handler
        .add_event_handler(TransferHandler)
        .add_side_effect_handler(TransferSideEffectHandler)
        // add migration for the state's DB schema
        .add_state_migrations(NftMigrations)
        // add contract address for BAYC
        .add_address(
            "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D",
            &ChainId::Mainnet,
            17773490,
        )
        // add contract address for Doodles
        .add_address(
            "0x8a90CAb2b38dba80c64b7734e58Ee1dB38B8992e",
            &ChainId::Mainnet,
            17769635,
        );

    // Setup indexing config
    let config = Config::new(
        // Database
        PostgresRepo::new(&get_database_url()),
    )
    .with_blocks_per_batch(400)
    // Add all possible chains in your Dapp
    .add_chain(Chain::new(ChainId::Mainnet, &get_mainnet_json_rpc_url()))
    // add BAYC's and Doodles' contracts
    .add_contract(nfts_contract)
    // Reset to force recreation of tables with the fixed migration logic
    .reset(2);

    println!("Chaindexing is taking a moment to setup...");
    // Start Indexing Process
    chaindexing::index_states(&config).await.unwrap();
    println!("Chaindexing is indexing states for BAYC and Doodles contracts...");
    println!("Query 'nfts' table using 'SELECT * from nfts' to see populated indices.");

    loop {
        // Infinite loop to keep the main thread running
        std::thread::sleep(std::time::Duration::from_millis(500))
    }
}

fn get_database_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn get_mainnet_json_rpc_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("MAINNET_JSON_RPC_URL").expect("MAINNET_JSON_RPC_URL must be set")
}
