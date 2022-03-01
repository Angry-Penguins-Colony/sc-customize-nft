#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use alloc::{borrow::ToOwned, format};
use elrond_wasm::{elrond_codec::TopEncode, String};

use super::{item::Item, item_slot::ItemSlot};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(PartialEq, TypeAbi, Debug)]
pub struct PenguinAttributes<M: ManagedTypeApi> {
    pub hat: Option<Item<M>>,
    pub background: Option<Item<M>>,
    pub skin: Option<Item<M>>,
    pub beak: Option<Item<M>>,
    pub weapon: Option<Item<M>>,
    pub clothes: Option<Item<M>>,
    pub eye: Option<Item<M>>,
}

impl<M: ManagedTypeApi> TopDecode for PenguinAttributes<M> {
    const TYPE_INFO: elrond_codec::TypeInfo = elrond_codec::TypeInfo::Unknown;

    fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let mut penguin = PenguinAttributes::empty();

        let boxed_slice_u8 = input.into_boxed_slice_u8();
        let x = boxed_slice_u8.split(|b| *b == b';');

        for item_str in x {
            let mut parts = item_str.split(|b| *b == b':');

            let slot = parts.next().unwrap().to_owned();
            let item_str = parts.next().unwrap().to_owned();

            let item = match item_str == b"unequipped" {
                true => None,
                false => Some(Item::new(&item_str)),
            };

            match slot.to_owned().as_slice() {
                b"Hat" => penguin.hat = item,
                b"Background" => penguin.background = item,
                b"Skin" => penguin.skin = item,
                b"Beak" => penguin.beak = item,
                b"Weapon" => penguin.weapon = item,
                b"Clothes" => penguin.clothes = item,
                b"Eyes" => penguin.eye = item,
                _ => return Result::Err(DecodeError::UTF8_DECODE_ERROR),
            };
        }

        return Result::Ok(penguin);
    }
}

impl<M: ManagedTypeApi> TopEncode for PenguinAttributes<M> {
    const TYPE_INFO: elrond_codec::TypeInfo = elrond_codec::TypeInfo::Unknown;

    fn top_encode<O: elrond_codec::TopEncodeOutput>(
        &self,
        output: O,
    ) -> Result<(), elrond_codec::EncodeError> {
        let mut managed_buffer = ManagedBuffer::<M>::new();

        for (i, slot) in ItemSlot::VALUES.iter().enumerate() {
            managed_buffer.append(&self.to_managed_buffer(slot));

            // add comma, except for the last line
            if i < ItemSlot::VALUES.len() - 1 {
                managed_buffer.append_bytes(b";");
            }
        }

        output.set_boxed_bytes(managed_buffer.to_boxed_bytes().into_box());
        return Result::Ok(());
    }
}

impl<M: ManagedTypeApi> PenguinAttributes<M> {
    pub fn new(args: &[(&ItemSlot, Item<M>)]) -> Self {
        let mut attributes = Self::empty();

        for (slot, item) in args {
            let result = attributes.set_item(slot, Option::Some(item.clone()));

            if result.is_err() {
                panic!("Failed to set item on slot");
            }
        }

        return attributes;
    }

    pub fn set_item(
        &mut self,
        slot: &ItemSlot,
        item: Option<Item<M>>,
    ) -> Result<(), ManagedBuffer<M>> {
        if self.is_slot_empty(slot) == false {
            return Result::Err(ManagedBuffer::new_from_bytes(
                b"The slot is not empty. Please free it, before setting an item.",
            ));
        }

        return self.__set_item_no_check(slot, item);
    }

    pub fn get_fill_count(&self) -> usize {
        let mut size: usize = 0;

        for slot in ItemSlot::VALUES.iter() {
            if self.is_slot_empty(slot) == false {
                size += 1;
            }
        }

        return size;
    }

    #[allow(unreachable_patterns)]
    pub fn get_item(&self, slot: &ItemSlot) -> Option<Item<M>> {
        match slot {
            &ItemSlot::Hat => return self.hat.clone(),
            &ItemSlot::Background => return self.background.clone(),
            &ItemSlot::Skin => return self.skin.clone(),
            &ItemSlot::Beak => return self.beak.clone(),
            &ItemSlot::Weapon => return self.weapon.clone(),
            &ItemSlot::Clothes => return self.clothes.clone(),
            &ItemSlot::Eye => return self.eye.clone(),
            _ => panic!("Missing slot. Please add it in get_item"),
        };
    }

    #[allow(unreachable_patterns)]
    pub fn is_slot_empty(&self, slot: &ItemSlot) -> bool {
        let item = self.get_item(slot);

        match item {
            Some(_) => false,
            None => true,
        }
    }

    pub fn set_empty_slot(&mut self, slot: &ItemSlot) -> Result<(), ManagedBuffer<M>> {
        return self.__set_item_no_check(slot, Option::None);
    }

    pub fn empty() -> Self {
        Self {
            hat: Option::None,
            background: Option::None,
            skin: Option::None,
            beak: Option::None,
            weapon: Option::None,
            clothes: Option::None,
            eye: Option::None,
        }
    }

    /// Set an item on a slot, without checking if the slot is empty.
    fn __set_item_no_check(
        &mut self,
        slot: &ItemSlot,
        item: Option<Item<M>>,
    ) -> Result<(), ManagedBuffer<M>> {
        #[allow(unreachable_patterns)]
        match slot {
            ItemSlot::Hat => self.hat = item,
            ItemSlot::Background => self.background = item,
            ItemSlot::Skin => self.skin = item,
            ItemSlot::Beak => self.beak = item,
            ItemSlot::Weapon => self.weapon = item,
            ItemSlot::Clothes => self.clothes = item,
            ItemSlot::Eye => self.eye = item,
            _ => panic!("Missing slot. Please add it in get_item"),
        }

        Result::Ok(())
    }

    fn to_managed_buffer(&self, slot: &ItemSlot) -> ManagedBuffer<M> {
        let mut slot_str = slot.to_bytes::<M>().to_owned();
        slot_str[0] = slot_str.to_ascii_uppercase()[0].to_owned();

        let item = match self.get_item(slot) {
            Some(item) => {
                let mut output = ManagedBuffer::new();

                output.append(&item.name);
                output.append_bytes(b" (");

                output.append(&item.token.as_managed_buffer().clone());
                output.append_bytes(b"-");
                output.append(&self.u64_to_hex(&item.nonce));
                output.append_bytes(b")");

                output
            }
            None => ManagedBuffer::<M>::new_from_bytes(b"unequipped"),
        };

        let mut managed_buffer = ManagedBuffer::<M>::new();
        managed_buffer.append_bytes(slot_str.as_slice());
        managed_buffer.append_bytes(b":");
        managed_buffer.append_bytes(item.to_boxed_bytes().as_slice());

        return managed_buffer;
    }

    fn u64_to_hex(&self, val: &u64) -> ManagedBuffer<M> {
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
