use chaindexing::states::{ContractState, Filters, MultiChainState, Updates};
use chaindexing::{EventContext, EventHandler};

use crate::states::{Pool, TokenSwapVolume};

pub struct PoolCreatedEventHandler;

#[chaindexing::augmenting_std::async_trait]
impl EventHandler for PoolCreatedEventHandler {
    fn abi(&self) -> &'static str {
        "PoolCreated(address indexed token0, address indexed token1, uint24 indexed fee, int24 tickSpacing, address pool)"
    }

    async fn handle_event<'a, 'b>(&self, context: EventContext<'a, 'b>) {
        let event_params = context.get_event_params();

        // Extract each parameter as exactly specified in the ABI:
        let token0_address = event_params.get_address_string("token0");
        let token1_address = event_params.get_address_string("token1");
        let pool_contract_address = event_params.get_address_string("pool");
        let fee = event_params.get_u32("fee");
        let tick_spacing = event_params.get_i32("tickSpacing");

        Pool {
            token0_address,
            token1_address,
            pool_contract_address: pool_contract_address.clone(),
            fee,
            tick_spacing,
        }
        .create(&context)
        .await;

        // Include new UniswapV3Pool contract:{pool_contract_address} for indexing...
        chaindexing::include_contract(&context, "UniswapV3Pool", &pool_contract_address).await;
    }
}

pub struct SwapEventHandler;

#[chaindexing::augmenting_std::async_trait]
impl EventHandler for SwapEventHandler {
    fn abi(&self) -> &'static str {
        "Swap(address indexed sender, address indexed recipient, int256 amount0, int256 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick)"
    }

    async fn handle_event<'a, 'b>(&self, context: EventContext<'a, 'b>) {
        let event_params = context.get_event_params();
        let event = &context.event;

        let pool = Pool::read_one(
            &Filters::new("pool_contract_address", &event.contract_address).within_multi_chain(),
            &context,
        )
        .await
        .unwrap();

        // Extract each parameter as exactly specified in the ABI:
        let amount0 = event_params.get_int_ether("amount0");
        let amount1 = event_params.get_int_ether("amount1");

        let token0 = pool.token0_address;
        let token1 = pool.token1_address;

        let last_updated_at = event.get_block_timestamp();

        create_or_update_token_swap_volume(token0, amount0, last_updated_at, &context).await;
        create_or_update_token_swap_volume(token1, amount1, last_updated_at, &context).await;
    }
}

async fn create_or_update_token_swap_volume<'a, 'b>(
    token_address: String,
    amount_ether: f64,
    last_updated_at: u64,
    context: &EventContext<'a, 'b>,
) {
    let amount_ether = amount_ether.abs(); // Volume calculation is additive

    if let Some(prev_token_swap_volume) = TokenSwapVolume::read_one(
        &Filters::new("token_address", &token_address).within_multi_chain(),
        context,
    )
    .await
    {
        let new_amount_ether =
            prev_token_swap_volume.amount_ether.parse::<f64>().unwrap() + amount_ether;
        let updates =
            Updates::new("amount_ether", new_amount_ether).add("last_updated_at", last_updated_at);

        prev_token_swap_volume.update(&updates, context).await;
    } else {
        TokenSwapVolume {
            token_address,
            amount_ether: amount_ether.to_string(),
            last_updated_at,
        }
        .create(context)
        .await;
    }
}
