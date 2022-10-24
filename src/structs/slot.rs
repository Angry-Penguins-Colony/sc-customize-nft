use crate::utils::managed_buffer_utils::ManagedBufferUtils;
use core::cmp::Ordering;
use elrond_wasm::elrond_codec::TopEncode;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const ERR_UNSUPPORTED_CHARACTERS: &[u8] =
    b"A slot can't contains colon or semicolon characters.";

pub const ERR_MUST_BE_LOWERCASE: &[u8] = b"The slot must be in lowercase";

#[derive(ManagedVecItem, TypeAbi, Clone, Debug)]
pub struct Slot<M: ManagedTypeApi> {
    slot: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> Default for Slot<M> {
    fn default() -> Self {
        Self {
            slot: Default::default(),
        }
    }
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
        if slot.contains_char(b';') || slot.contains_char(b':') {
            M::error_api_impl().signal_error(ERR_UNSUPPORTED_CHARACTERS);
        }

        if slot.is_lowercase() == false {
            M::error_api_impl().signal_error(ERR_MUST_BE_LOWERCASE);
        }

        Self {
            slot: slot.to_lowercase(),
        }
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        Self::new_from_buffer(ManagedBuffer::new_from_bytes(bytes))
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        self.slot.compare(&other.slot)
    }

    pub fn get(&self) -> ManagedBuffer<M> {
        self.slot.clone()
    }
}
