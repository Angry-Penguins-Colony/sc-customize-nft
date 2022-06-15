#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use core::ops::Deref;

use alloc::{borrow::ToOwned, format};
use elrond_wasm::elrond_codec::{TopDecodeInput, TopEncode};

use crate::utils::{self, split_buffer};

use super::item::Item;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, PartialEq, TypeAbi, Debug)]
pub struct PenguinAttributes<M: ManagedTypeApi> {
    slots: ManagedVec<M, ManagedBuffer<M>>,
    items: ManagedVec<M, Item<M>>,
}

impl<M: ManagedTypeApi> TopDecode for PenguinAttributes<M> {
    fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let unequipped_buffer = ManagedBuffer::<M>::new_from_bytes(b"unequipped");

        let mut penguin = PenguinAttributes::empty();

        let buffer = <ManagedBuffer<M> as TopDecode>::top_decode(input).unwrap();
        let items_raw = split_buffer(&buffer, b';');

        for item_raw in items_raw.iter() {
            let parts = split_buffer(item_raw.deref(), b':');

            let slot = parts.get(0).deref().to_owned();
            let item_buffer = parts.get(1);

            let item = if item_buffer.deref() == &unequipped_buffer {
                None
            } else {
                Some(Item::top_decode(item_buffer.deref()).unwrap())
            };

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

        for (i, slot) in self.slots.iter().enumerate() {
            managed_buffer.append(&self.to_kvp_buffer(slot.deref()));

            // add comma, except for the last line
            if i < self.items.len() - 1 {
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
    pub fn new(items_by_slot: &[(&ManagedBuffer<M>, Item<M>)]) -> Self {
        let mut attributes = Self::empty();

        for (slot, item) in items_by_slot {
            attributes.set_item(slot, Option::Some(item.clone()));
        }

        return attributes;
    }

    pub fn get_item(&self, slot: &ManagedBuffer<M>) -> Option<Item<M>> {
        match self.__get_index(slot) {
            Some(index) => Option::Some(self.items.get(index)),
            None => None,
        }
    }

    pub fn set_item(&mut self, slot: &ManagedBuffer<M>, item: Option<Item<M>>) {
        if self.is_slot_empty(slot) == false {
            panic!("The slot is not empty. Please free it, before setting an item.");
        }

        return self.__set_item_no_check(slot, item);
    }

    pub fn get_count(&self) -> usize {
        return self.items.len();
    }

    pub fn is_slot_empty(&self, slot: &ManagedBuffer<M>) -> bool {
        let item = self.get_item(slot);

        match item {
            Some(_) => false,
            None => true,
        }
    }

    pub fn empty_slot(&mut self, slot: &ManagedBuffer<M>) {
        return self.__set_item_no_check(slot, Option::None);
    }

    pub fn empty() -> Self {
        return PenguinAttributes {
            slots: ManagedVec::new(),
            items: ManagedVec::new(),
        };
    }

    pub fn to_kvp_buffer(&self, slot: &ManagedBuffer<M>) -> ManagedBuffer<M> {
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
        managed_buffer.append(&utils::capitalize(slot));
        managed_buffer.append_bytes(b":");
        managed_buffer.append(&item);

        return managed_buffer;
    }

    fn __get_index(&self, slot: &ManagedBuffer<M>) -> Option<usize> {
        return self.slots.iter().position(|s| s.deref() == slot);
    }

    /// Set an item on a slot, without checking if the slot is empty.
    fn __set_item_no_check(&mut self, slot: &ManagedBuffer<M>, item: Option<Item<M>>) {
        let index = self.__get_index(slot);

        match index {
            Some(index) => {
                if let Some(item) = item {
                    let result = self.items.set(index, &item);

                    if result.is_err() {
                        panic!("Failed to set item: {:?}", result.err());
                    }
                } else {
                    self.items.remove(index);
                    self.slots.remove(index);
                }
            }
            None => match item {
                Some(item) => {
                    self.slots.push(slot.clone());
                    self.items.push(item);
                }
                None => {
                    // Try to remove a None item, so we do nothing
                }
            },
        }
    }
}
