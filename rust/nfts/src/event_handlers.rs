use chaindexing::states::{ContractState, Filters, Updates};
use chaindexing::{
    EventContext, EventHandler, HandlerResult, SideEffectContext, SideEffectHandler,
};

use crate::states::Nft;

pub struct TransferHandler;

#[chaindexing::augmenting_std::async_trait]
impl EventHandler for TransferHandler {
    fn abi(&self) -> &'static str {
        "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)"
    }

    async fn handle_event<'a, 'b>(&self, context: EventContext<'a, 'b>) -> HandlerResult {
        let event_params = context.get_event_params();

        // Extract each parameter as exactly specified in the ABI:
        // "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)"
        let _from = event_params.get_address_string("from");
        let to = event_params.get_address_string("to");
        let token_id = event_params.get_u32("tokenId");

        if let Some(existing_nft) =
            Nft::read_one(&Filters::new("token_id", token_id), &context).await?
        {
            let updates = Updates::new("owner_address", &to);
            existing_nft.update(&updates, &context).await?;
        } else {
            let new_nft = Nft {
                token_id,
                owner_address: to,
            };

            new_nft.create(&context).await?;
        }

        Ok(())
    }
}

pub struct TransferSideEffectHandler;

#[chaindexing::augmenting_std::async_trait]
impl SideEffectHandler for TransferSideEffectHandler {
    type SharedState = ();

    fn abi(&self) -> &'static str {
        "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)"
    }

    async fn handle_event<'a>(
        &self,
        context: SideEffectContext<'a, Self::SharedState>,
    ) -> HandlerResult {
        let token_id = context.get_event_params().get_u32("tokenId");

        context
            .enqueue_outbox(
                "nft-transfer-notification",
                &format!("token {token_id} moved"),
            )
            .await?;

        Ok(())
    }
}
