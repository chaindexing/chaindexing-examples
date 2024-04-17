use chaindexing::states::{ContractState, MultiChainState, StateMigrations};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pool {
    pub token0_address: String,
    pub token1_address: String,
    pub fee: u32,
    pub tick_spacing: i32,
    pub pool_contract_address: String,
}

impl ContractState for Pool {
    fn table_name() -> &'static str {
        "uniswap_pools"
    }
}

pub struct PoolMigrations;

impl StateMigrations for PoolMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS uniswap_pools (
                token0_address VARCHAR NOT NULL,
                token1_address VARCHAR NOT NULL,
                pool_contract_address VARCHAR NOT NULL,
                fee INTEGER NOT NULL,
                tick_spacing INTEGER NOT NULL,
            )",
        ]
    }
}

/// Volumes swapped on Uniswap per token address
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenSwapVolume {
    pub token_address: String,
    pub amount_ether: String,
    pub last_updated_at: u64,
}

impl MultiChainState for TokenSwapVolume {
    fn table_name() -> &'static str {
        "uniswap_token_swap_volumes"
    }
}

pub struct TokenSwapVolumeMigrations;

impl StateMigrations for TokenSwapVolumeMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS uniswap_token_swap_volumes (
                token_address VARCHAR NOT NULL,
                amount_ether VARCHAR NOT NULL,
                last_updated_at BIGINT NOT NULL
            )",
        ]
    }
}
