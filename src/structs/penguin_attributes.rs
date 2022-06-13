#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use core::ops::Deref;

use alloc::{borrow::ToOwned, format};
use elrond_wasm::{
    elrond_codec::{TopDecodeInput, TopEncode},
    String,
};

use super::{item::Item, item_slot::ItemSlot, utils::split_buffer};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, PartialEq, TypeAbi, Debug)]
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
    fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let unequipped_buffer = ManagedBuffer::<M>::new_from_bytes(b"unequipped");

        let mut penguin = PenguinAttributes::empty();

        let buffer = <ManagedBuffer<M> as TopDecode>::top_decode(input).unwrap();
        let items_raw = split_buffer(&buffer, b';');

        for item_raw in items_raw.iter() {
            let parts = split_buffer(item_raw.deref(), b':');

            let slot_buffer = parts.get(0).deref().to_owned();
            let item_buffer = parts.get(1);

            let item = if item_buffer.deref() == &unequipped_buffer {
                None
            } else {
                Some(Item::top_decode(item_buffer.deref()).unwrap())
            };

            let slot = ItemSlot::from(slot_buffer);

            if slot == ItemSlot::None {
                return Result::Err(DecodeError::from(&b"Unable to parse a slot"[..]));
            }

            let _ = penguin.set_item(&slot, item);
        }

        return Result::Ok(penguin);
    }
}

impl<M: ManagedTypeApi> TopEncode for PenguinAttributes<M> {
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

        let mut bytes: [u8; 512] = [0; 512];
        managed_buffer.load_to_byte_array(&mut bytes);
        output.set_slice_u8(&bytes[..managed_buffer.len()]);

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
        let item = match self.get_item(slot) {
            Some(item) => {
                let mut output = ManagedBuffer::new();

                // TODO: optimize Item.top_encode
                // TODO: we unwrap to trigger if top_encode fails, but there must be a better way
                item.top_encode(&mut output).unwrap();

                output
            }
            None => ManagedBuffer::<M>::new_from_bytes(b"unequipped"),
        };

        let mut managed_buffer = ManagedBuffer::<M>::new();
        managed_buffer.append_bytes(slot.to_title_bytes::<M>());
        managed_buffer.append_bytes(b":");
        managed_buffer.append(&item);

        return managed_buffer;
    }
}
