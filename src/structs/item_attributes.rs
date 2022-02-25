#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Debug)]
pub struct ItemAttributes<M: ManagedTypeApi> {
    pub item_id: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ItemAttributes<M> {
    pub fn random() -> Self {
        ItemAttributes {
            item_id: ManagedBuffer::new_random(4),
        }
    }
}