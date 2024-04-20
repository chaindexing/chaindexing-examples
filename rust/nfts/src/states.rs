use chaindexing::augmenting_std::serde::{Deserialize, Serialize};
use chaindexing::states::{ContractState, StateMigrations};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "chaindexing::augmenting_std::serde")]
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
