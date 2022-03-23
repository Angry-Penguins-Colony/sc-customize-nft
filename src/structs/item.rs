#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use alloc::{borrow::ToOwned, format, string::ToString};
use elrond_wasm::{elrond_codec::TopDecodeInput, String};

use crate::structs::utils::{remove_first_and_last_char, split_last_occurence};

use super::{
    item_slot::ItemSlot,
    utils::{hex_to_u64, remove_first_char},
};
use core::{ops::Deref, str::FromStr};

use super::utils::split_buffer;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Debug)]
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

        managed_buffer.append(&self.name);
        managed_buffer.append_bytes(b" (");

        managed_buffer.append(&self.token.as_managed_buffer());
        managed_buffer.append_bytes(b"-");
        managed_buffer.append(&Item::u64_to_hex(&self.nonce)); // REMOVE: alloc+format here
        managed_buffer.append_bytes(b")");

        output.set_boxed_bytes(managed_buffer.into_boxed_slice_u8()); // REMOVE: ALLOC HERE
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

    fn split_last_occurence(bytes: &[u8], char: u8) -> (&[u8], &[u8]) {
        for i in (0..bytes.len() - 1).rev() {
            if bytes[i] == char {
                return bytes.split_at(i);
            }
        }

        panic!("no occurence of char {} in bytes {:?}", char, bytes);
    }

    pub fn u64_to_hex(val: &u64) -> ManagedBuffer<M> {
        let hex_val = format!("{:x}", val); // TODO: remove screen + format
        let bytes = hex_val.as_bytes();

        // make hex odd
        if &bytes.len() % 2 != 0 {
            let mut o = ManagedBuffer::<M>::new();

            o.append_bytes(b"0");
            o.append_bytes(bytes);

            return o;
        } else {
            return ManagedBuffer::<M>::new_from_bytes(bytes);
        }
    }
}
