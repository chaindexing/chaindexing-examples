use chaindexing::{ContractState, EventContext, EventHandler};

use crate::states::Nft;

pub struct TransferEventHandler;

#[async_trait::async_trait]
impl EventHandler for TransferEventHandler {
    type SharedState = ();

    async fn handle_event<'a, 'b>(&self, event_context: EventContext<'a, 'b, Self::SharedState>) {
        let event_params = event_context.get_event_params();

        // Extract each parameter as exactly specified in the ABI:
        // "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)"
        let _from = event_params.get_address_string("from");
        let to = event_params.get_address_string("to");
        let token_id = event_params.get_u32("tokenId");

        if let Some(nft_state) =
            Nft::read_one([("token_id", token_id.to_string())].into(), &event_context).await
        {
            let updates = [("owner_address", to.clone())];
            nft_state.update(updates.into(), &event_context).await;
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
