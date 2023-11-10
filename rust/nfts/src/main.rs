mod event_handlers;
mod states;

use std::collections::HashMap;

use chaindexing::{Chain, Chaindexing, Chains, Config, Contract, PostgresRepo, Repo};
use event_handlers::TransferEventHandler;
use states::NftStateMigrations;

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
        .add_state_migrations(NftStateMigrations)
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
        PostgresRepo::new("postgres://postgres:postgres@localhost/example-db"),
        // All possible chains in your Dapp
        supported_chains(),
    )
    // add BAYC's and Doodles' contracts
    .add_contract(bayc_contract)
    .add_contract(doodles_contract);

    // Start Indexing Process
    Chaindexing::index_states(&config).await.unwrap();
}

fn supported_chains() -> Chains {
    HashMap::from([(Chain::Mainnet, mainnet_json_rpc_url())])
}

fn mainnet_json_rpc_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("MAINNET_JSON_RPC_URL").expect("MAINNET_JSON_RPC_URL must be set")
}
