#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Debug)]
pub struct ItemAttributes {}
