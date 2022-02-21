#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use super::item_slot::ItemSlot;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Debug)]
pub struct Item<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub nonce: u64,
}
