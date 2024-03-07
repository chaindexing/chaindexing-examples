use chaindexing::{ContractState, ContractStateMigrations};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Nft {
    pub token_id: i32,
    pub contract_address: String,
    pub owner_address: String,
}

impl ContractState for Nft {
    fn table_name() -> &'static str {
        "nfts"
    }
}

pub struct NftMigrations;

impl ContractStateMigrations for NftMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS nfts (
                token_id INTEGER NOT NULL,
                contract_address TEXT NOT NULL,
                owner_address TEXT NOT NULL
            )",
        ]
    }
}
