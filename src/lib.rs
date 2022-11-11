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

use crate::{constants::*, structs::token::Token};

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

            let item = Item { name, slot };
            let token = Token::new(token_id, token_nonce);

            self.insert_or_replace_item_token(item, token);
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
    #[endpoint(claimItems)]
    fn claims_items(&self) {
        let owner = &self.blockchain().get_owner_address();
        let contract = &self.blockchain().get_sc_address();

        for (_, token) in self.map_items_tokens().iter() {
            let balance = &self
                .blockchain()
                .get_esdt_balance(contract, &token.token, token.nonce);

            if balance > &0 {
                self.send()
                    .direct_esdt(owner, &token.token, token.nonce, balance, b"");
            }
        }
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
