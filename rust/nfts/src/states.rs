use chaindexing::states::{ContractState, StateMigrations};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Nft {
    pub token_id: u32,
    pub owner_address: String,
}

impl ContractState for Nft {
    fn table_name() -> &'static str {
        "nfts"
    }
}

pub struct NftMigrations;

impl StateMigrations for NftMigrations {
    fn migrations(&self) -> &'static [&'static str] {
        &["CREATE TABLE IF NOT EXISTS nfts (
                token_id INTEGER NOT NULL,
                owner_address TEXT NOT NULL
            )"]
    }
}
