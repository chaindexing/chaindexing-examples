use chaindexing::states::{ContractState, Filters};
use chaindexing::{EventContext, EventHandler, SideEffectContext, SideEffectHandler};

use crate::states::Nft;

pub struct TransferHandler;

#[async_trait::async_trait]
impl EventHandler for TransferHandler {
    fn abi(&self) -> &'static str {
        "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)"
    }

    async fn handle_event<'a, 'b>(&self, event_context: EventContext<'a, 'b>) {
        let event_params = event_context.get_event_params();

        // Extract each parameter as exactly specified in the ABI:
        // "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)"
        let _from = event_params.get_address_string("from");
        let to = event_params.get_address_string("to");
        let token_id = event_params.get_u32("tokenId");

        if let Some(nft_state) =
            Nft::read_one(&Filters::new("token_id", token_id), &event_context).await
        {
            nft_state
                .update(&Filters::new("owner_address", to.clone()), &event_context)
                .await;
        } else {
            Nft {
                token_id,
                owner_address: to,
            }
            .create(&event_context)
            .await;
        }
    }
}

pub struct TransferSideEffectHandler;

#[async_trait::async_trait]
impl SideEffectHandler for TransferSideEffectHandler {
    type SharedState = ();

    fn abi(&self) -> &'static str {
        "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)"
    }

    async fn handle_event<'a>(&self, _event_context: SideEffectContext<'a, Self::SharedState>) {
        // println!("Handling side effect...")
    }
}
