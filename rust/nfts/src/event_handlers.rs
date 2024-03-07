use chaindexing::utils::address_to_string;
use chaindexing::{ContractState, EventContext, EventHandler};

use crate::states::Nft;

pub struct TransferEventHandler;

#[async_trait::async_trait]
impl EventHandler for TransferEventHandler {
    type SharedState = ();

    async fn handle_event<'a>(&self, event_context: EventContext<'a, Self::SharedState>) {
        let event = &event_context.event;
        // Get event parameters
        let event_params = event.get_params();

        // Extract each parameter as exactly specified in the ABI:
        // "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)"
        // Use address_to_string util to avoid truncation
        let from = address_to_string(
            &event_params
                .get("from")
                .unwrap()
                .clone()
                .into_address()
                .unwrap(),
        );
        let to = address_to_string(
            &event_params
                .get("to")
                .unwrap()
                .clone()
                .into_address()
                .unwrap(),
        );
        let token_id = event_params
            .get("tokenId")
            .unwrap()
            .clone()
            .into_uint()
            .unwrap();

        if let Some(nft_state) = Nft::read_one(
            [
                ("token_id".to_owned(), token_id.to_string()),
                ("contract_address".to_owned(), from.to_string()),
            ]
            .into(),
            &event_context,
        )
        .await
        {
            let updates = [("owner_address".to_string(), to)];

            nft_state.update(updates.into(), &event_context).await;
        } else {
            Nft {
                token_id: token_id.as_u32() as i32,
                contract_address: event.contract_address.clone(),
                owner_address: to,
            }
            .create(&event_context)
            .await;
        }
    }
}
