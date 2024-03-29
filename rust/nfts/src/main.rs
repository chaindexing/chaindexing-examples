mod event_handlers;
mod states;

use chaindexing::{Chain, Chaindexing, Config, Contract, PostgresRepo, Repo};
use event_handlers::TransferEventHandler;
use states::NftMigrations;

#[tokio::main]
async fn main() {
    // Setup BAYC's contract
    let bayc_contract = Contract::new("BoredApeYachtClub")
        // add transfer event and its corresponding handler
        .add_event(
            "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)",
            TransferEventHandler,
        )
        // add migration for the state's DB schema
        .add_state_migrations(NftMigrations)
        // add contract address for BAYC
        .add_address(
            "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D",
            &Chain::Mainnet,
            17773490,
        );

    // Setup Doodles' contract
    let doodles_contract = Contract::new("Doodles")
        .add_event(
            "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)",
            TransferEventHandler,
        )
        .add_address(
            "0x8a90CAb2b38dba80c64b7734e58Ee1dB38B8992e",
            &Chain::Mainnet,
            17769635,
        );

    // Setup indexing config
    let config = Config::new(
        // Database
        PostgresRepo::new(&get_database_url()),
    )
    // Add all possible chains in your Dapp
    .add_chain(Chain::Mainnet, &get_mainnet_json_rpc_url())
    // add BAYC's and Doodles' contracts
    .add_contract(bayc_contract)
    .add_contract(doodles_contract);

    println!("Start indexing states with Chaindexing...");

    // Start Indexing Process
    Chaindexing::index_states(&config).await.unwrap();

    println!(
        "NFT States are being indexed. Check nft_states table to view the indices in real time"
    );

    loop {
        // Infinite loop to keep the main thread running
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
