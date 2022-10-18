#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use elrond_wasm::elrond_codec::TopDecodeInput;

use core::{ops::Deref, str::FromStr};

use crate::utils::{managed_buffer_utils::ManagedBufferUtils, u64_utils::UtilsU64};

use super::slot::Slot;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(ManagedVecItem, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone, Debug)]
pub struct Item<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub slot: Slot<M>,
}

impl<M: ManagedTypeApi> elrond_codec::TopEncode for Item<M> {
    const TYPE_INFO: elrond_codec::TypeInfo = elrond_codec::TypeInfo::Unknown;

    fn top_encode<O: elrond_codec::TopEncodeOutput>(
        &self,
        output: O,
    ) -> Result<(), elrond_codec::EncodeError> {
        let mut managed_buffer = ManagedBuffer::<M>::new();

        // build buffer
        managed_buffer.append(&self.slot.capitalized());
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

        if parts.len() != 2 {
            return Result::Err(DecodeError::INPUT_OUT_OF_RANGE);
        }

        return Result::Ok(Self {
            slot: Slot::new_from_buffer(parts.get(0).deref().clone()),
            name: parts.get(1).deref().clone(),
        });
    }
}
