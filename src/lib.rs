#![no_std]
#![no_main]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod constants;
pub mod libs;
pub mod structs;
pub mod utils;

use libs::*;
use structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item, slot::Slot};

use crate::constants::*;

#[elrond_wasm::derive::contract]
pub trait Equip: customize::CustomizeModule + storage::StorageModule {
    #[init]
    fn init(&self, equippable_token_id: TokenIdentifier) {
        self.equippable_token_id().set(&equippable_token_id);
    }

    #[endpoint(registerItem)]
    #[only_owner]
    fn register_item(
        &self,
        slot: Slot<Self::Api>,
        name: ManagedBuffer<Self::Api>,
        token_id: TokenIdentifier<Self::Api>,
        token_nonce: u64,
    ) {
        require!(
            token_id != self.equippable_token_id().get(),
            ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM
        );

        let item = &Item { name, slot };

        let storage_item = self.get_item_from_token(&token_id, token_nonce);
        let storage_token = self.get_token_from_item(item);

        require!(storage_item.is_empty(), ERR_CANNOT_OVERRIDE_REGISTERED_ITEM);
        require!(
            storage_token.is_empty(),
            ERR_CANNOT_OVERRIDE_REGISTERED_ITEM
        );

        storage_item.set(item);
        storage_token.set((token_id, token_nonce));
    }

    #[payable("*")]
    #[endpoint]
    #[only_owner]
    fn fill(&self) {
        let payment = self.call_value().single_esdt();

        require!(
            self.get_item_from_token(&payment.token_identifier, payment.token_nonce)
                .is_empty()
                == false,
            ERR_CANNOT_FILL_UNREGISTERED_ITEM
        )
    }

    /**
     * Endpoint of enqueue_image_to_render
     */
    #[endpoint(renderImage)]
    #[payable("EGLD")]
    fn add_image_to_render(&self, attributes: &EquippableNftAttributes<Self::Api>) {
        require!(
            self.call_value().egld_value() == BigUint::from(ENQUEUE_PRICE),
            ERR_PAY_0001_EGLD
        );

        self.enqueue_image_to_render(&attributes);
    }

    #[only_owner]
    #[endpoint(claim)]
    fn claim(&self) {
        let balance = self
            .blockchain()
            .get_balance(&self.blockchain().get_sc_address());

        self.send()
            .direct_egld(&self.blockchain().get_owner_address(), &balance, b"");
    }
}
