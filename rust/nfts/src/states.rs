use chaindexing::{ContractState, ContractStateMigrations};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NftState {
    pub token_id: i32,
    pub contract_address: String,
    pub owner_address: String,
}

impl ContractState for NftState {
    fn table_name() -> &'static str {
        "nft_states"
    }
}

pub struct NftStateMigrations;

impl ContractStateMigrations for NftStateMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS nft_states (
                token_id INTEGER NOT NULL,
                contract_address TEXT NOT NULL,
                owner_address TEXT NOT NULL
            )",
        ]
    }
}
