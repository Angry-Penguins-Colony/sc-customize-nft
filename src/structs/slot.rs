#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use elrond_wasm::elrond_codec::{TopDecodeInput, TopEncode};

use core::{cmp::Ordering, ops::Deref, str::FromStr};

use crate::{sc_panic_self, utils::managed_buffer_utils::ManagedBufferUtils};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const ERR_UNSUPPORTED_CHARACTERS: &str = "A slot can't contains colon or semicolon characters.";

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TypeAbi, Clone, Debug)]
pub struct Slot<M: ManagedTypeApi> {
    slot: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> PartialEq for Slot<M> {
    fn eq(&self, other: &Self) -> bool {
        self.slot == other.slot
    }
}

impl<M: ManagedTypeApi> TopDecode for Slot<M> {
    fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let input_buffer = <ManagedBuffer<M> as TopDecode>::top_decode(input).unwrap();

        return Result::Ok(Slot::new_from_buffer(input_buffer));
    }
}

impl<M: ManagedTypeApi> TopEncode for Slot<M> {
    fn top_encode<O: elrond_codec::TopEncodeOutput>(
        &self,
        output: O,
    ) -> Result<(), elrond_codec::EncodeError> {
        let managed_buffer = &self.slot;

        let bytes = managed_buffer.load_512_bytes();
        output.set_slice_u8(&bytes[..managed_buffer.len()]);

        return Result::Ok(());
    }
}

impl<M: ManagedTypeApi> Slot<M> {
    pub fn new_from_buffer(slot: ManagedBuffer<M>) -> Self {
        if slot.contains(b";") || slot.contains(b":") {
            sc_panic_self!(M, ERR_UNSUPPORTED_CHARACTERS)
        }

        Self {
            slot: slot.to_lowercase(),
        }
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        Self::new_from_buffer(ManagedBuffer::new_from_bytes(bytes))
    }

    pub fn capitalized(&self) -> ManagedBuffer<M> {
        self.slot.capitalize()
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        self.slot.compare(&other.slot)
    }
}
