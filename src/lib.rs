#![no_std]
#![no_main]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod constants;
pub mod libs;
pub mod structs;
pub mod utils;

use libs::*;
use structs::item::Item;

use crate::{
    constants::*,
    structs::{
        equippable_attributes::{
            panic_if_name_contains_unsupported_characters,
            panic_if_slot_contains_unsupported_characters,
        },
        token::Token,
    },
};

pub const ERR_BAD_ROYALTIES: &str = "The royalties must be between 0 and 10000";

#[elrond_wasm::derive::contract]
pub trait Equip:
    customize::CustomizeModule + storage::StorageModule + equippable_uris::EquippableUrisModule
{
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
            MultiValue4<ManagedBuffer<Self::Api>, ManagedBuffer, TokenIdentifier, u64>,
        >,
    ) {
        for item in items.into_iter() {
            let (slot, name, token_id, token_nonce) = item.into_tuple();

            require!(
                token_id != self.equippable_token_id().get(),
                ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM
            );

            panic_if_name_contains_unsupported_characters(&Option::Some(name.clone()));
            panic_if_slot_contains_unsupported_characters(&slot);

            let item = Item { name, slot };
            let token = Token::new(token_id, token_nonce);

            let is_insert_successful = self.map_items_tokens().insert(item, token);

            require!(is_insert_successful, ERR_CANNOT_OVERRIDE_REGISTERED_ITEM);
        }
    }

    #[payable("*")]
    #[endpoint]
    #[only_owner]
    fn fill(&self) {
        let payments = self.call_value().all_esdt_transfers();

        for payment in &payments {
            require!(
                self.has_token(&Token::new(payment.token_identifier, payment.token_nonce)),
                ERR_CANNOT_FILL_UNREGISTERED_ITEM
            )
        }
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

    #[only_owner]
    #[endpoint(overrideRoyalties)]
    fn override_royalties(&self, royalties: BigUint) {
        require!(royalties >= 0 && royalties <= 10000, ERR_BAD_ROYALTIES);

        self.royalties_overrided().set(&royalties);
    }

    #[view(getItems)]
    fn get_items(
        &self,
    ) -> MultiValueEncoded<
        MultiValue4<
            ManagedBuffer<Self::Api>,
            ManagedBuffer<Self::Api>,
            TokenIdentifier<Self::Api>,
            u64,
        >,
    > {
        let mut output = MultiValueEncoded::new();

        for (item, token) in self.map_items_tokens().iter() {
            let multi_value = MultiValue4::from((item.slot, item.name, token.token, token.nonce));
            output.push(multi_value);
        }

        return output;
    }
}
