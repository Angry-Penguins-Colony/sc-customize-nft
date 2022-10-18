use crate::{sc_panic_self, utils::managed_buffer_utils::ManagedBufferUtils};
use core::cmp::Ordering;
use elrond_wasm::elrond_codec::TopEncode;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const ERR_UNSUPPORTED_CHARACTERS: &str = "A slot can't contains colon or semicolon characters.";

#[derive(ManagedVecItem, TypeAbi, Clone, Debug)]
pub struct Slot<M: ManagedTypeApi> {
    slot: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> TopDecode for Slot<M> {
    fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let result = ManagedBuffer::top_decode(input);

        return match result {
            Result::Err(err) => Result::Err(err),
            Result::Ok(decoded_buffer) => Result::Ok(Slot::new_from_buffer(decoded_buffer)),
        };
    }
}

impl<M: ManagedTypeApi> NestedDecode for Slot<M> {
    const TYPE_INFO: elrond_codec::TypeInfo = elrond_codec::TypeInfo::Unknown;

    fn dep_decode<I: elrond_codec::NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        let result = ManagedBuffer::dep_decode(input);

        return match result {
            Result::Err(err) => Result::Err(err),
            Result::Ok(decoded_buffer) => Result::Ok(Slot::new_from_buffer(decoded_buffer)),
        };
    }
}

impl<M: ManagedTypeApi> TopEncode for Slot<M> {
    fn top_encode<O: elrond_codec::TopEncodeOutput>(
        &self,
        output: O,
    ) -> Result<(), elrond_codec::EncodeError> {
        let slot_lowercase = &self.slot.to_lowercase();

        return slot_lowercase.top_encode(output);
    }
}

impl<M: ManagedTypeApi> NestedEncode for Slot<M> {
    const TYPE_INFO: elrond_codec::TypeInfo = elrond_codec::TypeInfo::Unknown;

    fn dep_encode<O: elrond_codec::NestedEncodeOutput>(
        &self,
        dest: &mut O,
    ) -> Result<(), elrond_codec::EncodeError> {
        let slot_lowercase = &self.slot.to_lowercase();

        return slot_lowercase.dep_encode(dest);
    }
}

impl<M: ManagedTypeApi> PartialEq for Slot<M> {
    fn eq(&self, other: &Self) -> bool {
        self.slot == other.slot
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

    pub fn get(&self) -> ManagedBuffer<M> {
        self.slot.clone()
    }
}
