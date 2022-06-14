#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use alloc::{borrow::ToOwned, format, string::ToString};
use elrond_wasm::elrond_codec::TopDecodeInput;

use crate::structs::utils::{remove_first_and_last_char, split_last_occurence};

use super::utils::{hex_to_u64, remove_first_char, u64_to_hex};
use core::{ops::Deref, str::FromStr};

use super::utils::split_buffer;

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
        managed_buffer.append(&u64_to_hex(&self.nonce));
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
        let splited = split_last_occurence(&input, b' ');

        // part 1 build name
        let name = splited.0;

        // part 2: build identifier
        let identifier = remove_first_and_last_char(&splited.1); // remove parenthesis
        let (token, nonce) = split_last_occurence(&identifier, b'-');

        return Result::Ok(Self {
            token: TokenIdentifier::from_esdt_bytes(token),
            nonce: hex_to_u64(&nonce).unwrap(),
            name: name,
        });
    }
}
