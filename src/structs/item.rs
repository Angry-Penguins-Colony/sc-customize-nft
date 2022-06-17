#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use elrond_wasm::elrond_codec::TopDecodeInput;

use crate::utils::{ManagedBufferUtils, UtilsU64};
use core::{ops::Deref, str::FromStr};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Debug)]
pub struct Item<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub nonce: u64,
    pub name: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> elrond_codec::TopEncode for Item<M> {
    const TYPE_INFO: elrond_codec::TypeInfo = elrond_codec::TypeInfo::Unknown;

    fn top_encode<O: elrond_codec::TopEncodeOutput>(
        &self,
        output: O,
    ) -> Result<(), elrond_codec::EncodeError> {
        let mut managed_buffer = ManagedBuffer::<M>::new();

        // build buffer
        managed_buffer.append(&self.name);
        managed_buffer.append_bytes(b" (");

        managed_buffer.append(&self.token.as_managed_buffer());
        managed_buffer.append_bytes(b"-");
        managed_buffer.append(&self.nonce.to_hex());
        managed_buffer.append_bytes(b")");

        // set buffer to output
        let mut bytes: [u8; 512] = [0; 512];
        managed_buffer.load_to_byte_array(&mut bytes);
        output.set_slice_u8(&bytes[..managed_buffer.len()]);

        return Result::Ok(());
    }
}

impl<M: ManagedTypeApi> Item<M> {
    pub fn top_decode(input: &ManagedBuffer<M>) -> Result<Self, DecodeError> {
        let splited = input.split_last_occurence(b' ');

        // part 1 build name
        let name = splited.0;

        // part 2: build identifier
        let identifier = &splited.1.remove_first_and_last_char(); // removiing parenthesis
        let (token, nonce) = identifier.split_last_occurence(b'-');

        return Result::Ok(Self {
            token: TokenIdentifier::from_esdt_bytes(token),
            nonce: nonce.hex_to_u64().unwrap(),
            name: name,
        });
    }
}
