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
        let mut managed_buffer = ManagedBuffer::<M>::new();

        // build buffer
        managed_buffer.append(&self.slot.capitalize());
        managed_buffer.append_bytes(b":");
        managed_buffer.append(&self.name);

        // set buffer to output
        let mut bytes: [u8; 512] = [0; 512];
        managed_buffer.load_to_byte_array(&mut bytes);
        output.set_slice_u8(&bytes[..managed_buffer.len()]);

        return Result::Ok(());
    }
}

impl<M: ManagedTypeApi> Item<M> {
    pub fn top_decode(input: &ManagedBuffer<M>) -> Result<Self, DecodeError> {
        let parts = input.split(b':');

        return Result::Ok(Self {
            slot: parts.get(0).to_lowercase(),
            name: parts.get(1).deref().clone(),
        });
    }
}
