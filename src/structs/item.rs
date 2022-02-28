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
}

impl<M: ManagedTypeApi> Item<M> {
    pub fn new(bytes: &[u8]) -> Self {
        // get last index of "-" in bytes
        let last_index = bytes.iter().rposition(|b| *b == b'-').unwrap();

        // split str to last occurence of "-"
        let parts = bytes.split_at(last_index);

        let token = TokenIdentifier::from_esdt_bytes(parts.0);

        // remove first character of parts.1
        let nonce_str = String::from_utf8_lossy(&parts.1[1..])
            .to_owned()
            .to_string();
        let nonce = u64::from_str_radix(&nonce_str, 16).unwrap();

        Self { token, nonce }
    }
}
