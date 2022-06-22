#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use core::ops::Deref;

use elrond_wasm::{
    elrond_codec::{TopDecodeInput, TopEncode},
    formatter::SCDisplay,
};

use crate::{
    sc_panic_self,
    utils::{self, managed_buffer_utils::ManagedBufferUtils},
};

use super::item::Item;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, PartialEq, TypeAbi, Debug)]
pub struct EquippableNftAttributes<M: ManagedTypeApi> {
    pub slots: ManagedVec<M, ManagedBuffer<M>>,
    items: ManagedVec<M, Item<M>>,
}

impl<M: ManagedTypeApi> SCDisplay for EquippableNftAttributes<M> {
    fn fmt<F: elrond_wasm::formatter::FormatByteReceiver>(&self, f: &mut F) {
        let mut output = ManagedBuffer::<F::Api>::new_from_bytes(b"");

        let _ = self.top_encode(&mut output);
        f.append_managed_buffer(&output);
    }
}

impl<M: ManagedTypeApi> TopDecode for EquippableNftAttributes<M> {
    fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let mut equippable_attributes = EquippableNftAttributes::empty();

        let buffer = <ManagedBuffer<M> as TopDecode>::top_decode(input).unwrap();
        let items_raw = buffer.split(b';');

        for item_raw in items_raw.iter() {
            let parts = item_raw.deref().split(b':');

            let slot = parts.get(0).deref().clone();
            let item_buffer = parts.get(1);

            let item = Some(Item::top_decode(item_buffer.deref()).unwrap());

            equippable_attributes.set_item(&slot, item);
        }

        return Result::Ok(equippable_attributes);
    }
}

impl<M: ManagedTypeApi> TopEncode for EquippableNftAttributes<M> {
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

impl<M: ManagedTypeApi> EquippableNftAttributes<M> {
    pub fn new(items_by_slot: &[(&ManagedBuffer<M>, Item<M>)]) -> Self {
        let mut attributes = Self::empty();

        for (slot, item) in items_by_slot {
            attributes.set_item(&slot, Option::Some(item.clone()));
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
            sc_panic_self!(
                M,
                "The slot is not empty. Please free it, before setting an item."
            );
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
        return EquippableNftAttributes {
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
        managed_buffer.append(&slot.capitalize());
        managed_buffer.append_bytes(b":");
        managed_buffer.append(&item);

        return managed_buffer;
    }

    fn __get_index(&self, target_slot: &ManagedBuffer<M>) -> Option<usize> {
        return self
            .slots
            .iter()
            .position(|current_slot| current_slot.deref().equals_ignore_case(target_slot));
    }

    /// Set an item on a slot, without checking if the slot is empty.
    fn __set_item_no_check(&mut self, slot: &ManagedBuffer<M>, item: Option<Item<M>>) {
        let index = self.__get_index(slot);

        match index {
            Some(index) => {
                if let Some(item) = item {
                    let result = self.items.set(index, &item);

                    if result.is_err() {
                        sc_panic_self!(
                            M,
                            "Failed to set item, InvalidSliceError exception happened."
                        );
                    }
                } else {
                    self.items.remove(index);
                    self.slots.remove(index);
                }
            }
            None => match item {
                Some(item) => {
                    self.slots.push(slot.to_lowercase());
                    self.items.push(item);
                }
                None => {
                    // Try to remove a None item, so we do nothing
                }
            },
        }
    }
}
