#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use alloc::{borrow::ToOwned, format, string::ToString};
use elrond_wasm::{elrond_codec::TopDecodeInput, String};

use super::item_slot::ItemSlot;
use core::str::FromStr;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Debug)]
pub struct Item<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub nonce: u64,
    pub name: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> TopDecode for Item<M> {
    const TYPE_INFO: elrond_codec::TypeInfo = elrond_codec::TypeInfo::Unknown;

    fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        // format is "name (token-nonce)"

        // TODO: avoid into call
        let bytes = input.into_boxed_slice_u8(); // TODO: avoid Box, use instead ManagedByteArray
        let main_parts = Item::<M>::split_last_occurence(&bytes, b' ');

        // retrieve name
        let name = main_parts.0;

        // retrivier token
        let identifier = &main_parts.1[1..main_parts.1.len() - 1];
        let parts = Item::<M>::split_last_occurence(identifier, b'-');

        let token = TokenIdentifier::from_esdt_bytes(&parts.0[1..]);

        // retrieve nonce
        // TODO: avoid using String to decode nonce
        let nonce_str = String::from_utf8_lossy(&parts.1[1..])
            .to_owned()
            .to_string();
        let nonce = u64::from_str_radix(&nonce_str, 16).unwrap();

        return Result::Ok(Self {
            token,
            nonce,
            name: ManagedBuffer::<M>::new_from_bytes(name),
        });
    }
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
        managed_buffer.append(&Item::u64_to_hex(&self.nonce));
        managed_buffer.append_bytes(b")");

        // TODO: avoid into methods call
        output.set_boxed_bytes(managed_buffer.into_boxed_slice_u8());
        return Result::Ok(());
    }
}

impl<M: ManagedTypeApi> Item<M> {
    fn split_last_occurence(bytes: &[u8], char: u8) -> (&[u8], &[u8]) {
        for i in (0..bytes.len() - 1).rev() {
            if bytes[i] == char {
                return bytes.split_at(i);
            }
        }

        panic!("no occurence of char {} in bytes {:?}", char, bytes);
    }

    pub fn u64_to_hex(val: &u64) -> ManagedBuffer<M> {
        let hex_val = format!("{:x}", val);
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
