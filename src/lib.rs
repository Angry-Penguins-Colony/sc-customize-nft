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
        items: MultiValueEncoded<
            Self::Api,
            MultiValue4<Slot<Self::Api>, ManagedBuffer, TokenIdentifier, u64>,
        >,
    ) {
        for item in items.into_iter() {
            let (slot, name, token_id, token_nonce) = item.into_tuple();

            require!(
                token_id != self.equippable_token_id().get(),
                ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM
            );

            let is_insert_successful = self
                .map_items_tokens()
                .insert(Item { name, slot }, Token::new(token_id, token_nonce));

            require!(is_insert_successful, ERR_CANNOT_OVERRIDE_REGISTERED_ITEM);
        }
    }

    #[payable("*")]
    #[endpoint]
    #[only_owner]
    fn fill(&self) {
        // let payment = self.call_value().single_esdt();

        // require!(
        //     self.has_token(&Token::new(payment.token_identifier, payment.token_nonce)),
        //     ERR_CANNOT_FILL_UNREGISTERED_ITEM
        // )
    }

    /**
     * Endpoint of enqueue_image_to_render
     */
    #[endpoint(renderImage)]
    #[payable("EGLD")]
    fn enqueue_image_to_render(&self, attributes: &EquippableNftAttributes<Self::Api>) {
        require!(
            self.call_value().egld_value() == BigUint::from(ENQUEUE_PRICE),
            ERR_PAY_0001_EGLD
        );

        require!(
            self.uris_of_attributes(attributes).is_empty(),
            ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_ALREADY_RENDERED
        );
        require!(
            self.images_to_render().contains(attributes) == false,
            ERR_RENDER_ALREADY_IN_QUEUE
        );

        self.images_to_render().insert(attributes.clone());
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
