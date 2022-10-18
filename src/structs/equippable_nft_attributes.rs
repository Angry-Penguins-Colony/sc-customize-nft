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
    utils::{self, managed_buffer_utils::ManagedBufferUtils, managed_vec_utils::EqUtils},
};

use super::item::Item;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(ManagedVecItem, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone, Debug)]
struct EquippableNftAttribute<M: ManagedTypeApi> {
    pub slot: ManagedBuffer<M>,
    pub item: Option<Item<M>>,
}

impl<M: ManagedTypeApi> EquippableNftAttribute<M> {
    pub fn to_kvp_buffer(&self) -> ManagedBuffer<M> {
        let mut output_buffer = ManagedBuffer::<M>::new();

        let mut item_buffer = ManagedBuffer::new();
        match &self.item {
            Some(item) => {
                item.top_encode(&mut item_buffer).unwrap();
            }
            None => {
                let unequipped_item = &Item {
                    name: ManagedBuffer::new_from_bytes(b"unequipped"),
                    slot: self.slot.clone(),
                };

                unequipped_item.top_encode(&mut item_buffer).unwrap();
            }
        }
        output_buffer.append(&item_buffer);

        return output_buffer;
    }
}

#[derive(NestedEncode, NestedDecode, TypeAbi, Debug, Clone)]
pub struct EquippableNftAttributes<M: ManagedTypeApi> {
    kvp: ManagedVec<M, EquippableNftAttribute<M>>,
}

pub trait SortUtils<M>
where
    M: ManagedTypeApi,
{
    fn sort_alphabetically(&self) -> Self;
}
impl<M> SortUtils<M> for ManagedVec<M, EquippableNftAttribute<M>>
where
    M: ManagedTypeApi,
{
    fn sort_alphabetically(&self) -> Self {
        let mut remaining_items = self.clone();
        let mut output = Self::new();

        while remaining_items.len() > 0 {
            let mut smallest_item = remaining_items.get(0);
            let mut smallest_item_index = 0;

            for (index, kvp) in remaining_items.iter().enumerate() {
                if kvp.slot.compare(&smallest_item.slot).is_le() {
                    smallest_item = kvp;
                    smallest_item_index = index;
                }
            }

            output.push(smallest_item.clone());
            remaining_items.remove(smallest_item_index);
        }

        return output;
    }
}

impl<M: ManagedTypeApi + core::cmp::PartialEq> PartialEq for EquippableNftAttributes<M> {
    fn eq(&self, other: &Self) -> bool {
        return self.kvp.eq_unorder(&other.kvp);
    }
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
            let item = Item::top_decode(item_raw.deref()).unwrap();
            let slot = item.slot.clone();

            if &item.name == &ManagedBuffer::new_from_bytes(b"unequipped") {
                equippable_attributes.set_item_if_empty(&slot, None);
                continue;
            } else {
                equippable_attributes.set_item_if_empty(&slot, Some(item));
            }
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

        for (i, kvp) in self.kvp.iter().enumerate() {
            managed_buffer.append(&kvp.to_kvp_buffer());

            // add comma, except for the last line
            if i < self.kvp.len() - 1 {
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
    pub fn new(items_by_slot: &[Item<M>]) -> Self {
        let mut attributes = Self::empty();

        for item in items_by_slot {
            attributes.set_item_if_empty(&item.slot, Option::Some(item.clone()));
        }

        return attributes;
    }

    pub fn get_item(&self, slot: &ManagedBuffer<M>) -> Option<Item<M>> {
        match self.__get_index(slot) {
            Some(index) => self.kvp.get(index).item,
            None => None,
        }
    }

    pub fn set_item_if_empty(&mut self, slot: &ManagedBuffer<M>, item: Option<Item<M>>) {
        if self.is_slot_empty(slot) == false {
            sc_panic_self!(
                M,
                "The slot is not empty. Please free it, before setting an item."
            );
        }

        return self.set_item(slot, item);
    }

    pub fn get_count(&self) -> usize {
        let mut count = 0;

        for kvp in self.kvp.iter() {
            if kvp.item.is_some() {
                count = count + 1;
            }
        }

        return count;
    }

    pub fn is_slot_empty(&self, slot: &ManagedBuffer<M>) -> bool {
        let item = self.get_item(&slot.to_lowercase());

        match item {
            Some(_) => false,
            None => true,
        }
    }

    pub fn empty_slot(&mut self, slot: &ManagedBuffer<M>) {
        return self.set_item(&slot, Option::None);
    }

    pub fn empty() -> Self {
        return EquippableNftAttributes {
            kvp: ManagedVec::new(),
        };
    }

    fn __get_index(&self, slot: &ManagedBuffer<M>) -> Option<usize> {
        return self
            .kvp
            .iter()
            .position(|kvp| kvp.slot.equals_ignore_case(slot));
    }

    fn set_item(&mut self, slot: &ManagedBuffer<M>, item: Option<Item<M>>) {
        let slot = slot.to_lowercase();
        let index = self.__get_index(&slot);

        if item.is_some() && item.clone().unwrap().slot != slot {
            sc_panic_self!(M, "The item slot does not match the slot parameter.");
        }

        match index {
            Some(index) => {
                let result = self.kvp.set(index, &EquippableNftAttribute { item, slot });

                if result.is_err() {
                    sc_panic_self!(
                        M,
                        "Failed to set item, InvalidSliceError exception happened."
                    );
                }
            }
            None => {
                self.kvp.push(EquippableNftAttribute { slot, item });
            }
        }

        self.kvp = self.kvp.sort_alphabetically();
    }
}
