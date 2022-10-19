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

use crate::{constants::*, structs::token::Token};

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

        let item = Item { name, slot };
        let token = Token::new(token_id, token_nonce);

        require!(
            self.has_item(&item) == false,
            ERR_CANNOT_OVERRIDE_REGISTERED_ITEM
        );
        require!(
            self.has_token(&token) == false,
            ERR_CANNOT_OVERRIDE_REGISTERED_ITEM
        );

        self.map_items_tokens().insert(item, token);
    }

    #[payable("*")]
    #[endpoint]
    #[only_owner]
    fn fill(&self) {
        let payment = self.call_value().single_esdt();

        require!(
            self.has_token(&Token::new(payment.token_identifier, payment.token_nonce)),
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
