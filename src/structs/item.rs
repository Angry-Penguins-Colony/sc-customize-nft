#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use alloc::{borrow::ToOwned, format, string::ToString};
use elrond_wasm::String;

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
        // format is name (token-nonce)

        let bytes = input.into_boxed_slice_u8();
        let main_parts = Item::<M>::split_last_occurence(&bytes, b' ');

        // retrieve name
        let name = main_parts.0;

        // retrivier token
        let identifier = &main_parts.1[1..main_parts.1.len() - 1];
        let parts = Item::<M>::split_last_occurence(identifier, b'-');

        let token = TokenIdentifier::from_esdt_bytes(&parts.0[1..]);

        // retrieve nonce
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

        managed_buffer.append(&self.token.as_managed_buffer().clone());
        managed_buffer.append_bytes(b"-");
        managed_buffer.append(&Item::u64_to_hex(&self.nonce));
        managed_buffer.append_bytes(b")");

        output.set_boxed_bytes(managed_buffer.to_boxed_bytes().into_box());
        return Result::Ok(());
    }
}

impl<M: ManagedTypeApi> Item<M> {
    fn split_last_occurence(bytes: &[u8], char: u8) -> (&[u8], &[u8]) {
        let last_index = bytes.iter().rposition(|b| *b == char).unwrap();
        let parts = bytes.clone().split_at(last_index);
        return parts;
    }

    pub fn u64_to_hex(val: &u64) -> ManagedBuffer<M> {
        let hex_val = format!("{:x}", val);
        let bytes = hex_val.as_bytes();

        let mut o = ManagedBuffer::<M>::new();

        // make hex odd
        if &bytes.len() % 2 != 0 {
            o.append_bytes(b"0");
        }

        o.append_bytes(bytes);

        return o;
    }
}
