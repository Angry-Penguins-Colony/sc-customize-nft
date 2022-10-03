#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use elrond_wasm::elrond_codec::TopDecodeInput;

use core::{ops::Deref, str::FromStr};

use crate::utils::{managed_buffer_utils::ManagedBufferUtils, u64_utils::UtilsU64};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(ManagedVecItem, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone, Debug)]
pub struct Item<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub slot: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> elrond_codec::TopEncode for Item<M> {
    const TYPE_INFO: elrond_codec::TypeInfo = elrond_codec::TypeInfo::Unknown;

    fn top_encode<O: elrond_codec::TopEncodeOutput>(
        &self,
        output: O,
    ) -> Result<(), elrond_codec::EncodeError> {
        panic!("not implemented");
    }
}

impl<M: ManagedTypeApi> Item<M> {
    pub fn top_decode(input: &ManagedBuffer<M>) -> Result<Self, DecodeError> {
        panic!("not implemented");
    }
}
