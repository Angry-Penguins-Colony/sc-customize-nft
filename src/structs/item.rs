#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use alloc::{borrow::ToOwned, string::ToString};
use elrond_wasm::String;

use super::item_slot::ItemSlot;
use core::str::FromStr;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Debug)]
pub struct Item<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub nonce: u64,
    pub name: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> Item<M> {
    pub fn new(bytes: &[u8]) -> Self {
        // format is name (token-nonce)

        let main_parts = Item::<M>::split_last_occurence(bytes, b' ');

        // retrieve name
        let name = main_parts.0;

        // retrivier token
        let identifier = &main_parts.1[1..main_parts.1.len() - 1];
        let parts = Item::<M>::split_last_occurence(identifier, b'-');

        let token = TokenIdentifier::from_esdt_bytes(&parts.0[1..]);

        // retrieve nonce
        let nonce_str = String::from_utf8_lossy(&parts.1[1..])
            .to_owned()
            .to_string();
        let nonce = u64::from_str_radix(&nonce_str, 16).unwrap();

        Self {
            token,
            nonce,
            name: ManagedBuffer::<M>::new_from_bytes(name),
        }
    }

    fn split_last_occurence(bytes: &[u8], char: u8) -> (&[u8], &[u8]) {
        let last_index = bytes.iter().rposition(|b| *b == char).unwrap();
        let parts = bytes.clone().split_at(last_index);
        return parts;
    }
}
