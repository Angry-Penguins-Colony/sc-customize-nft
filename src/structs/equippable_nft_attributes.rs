use crate::{
    constants::UNEQUIPPED_ITEM_NAME,
    sc_panic_self,
    utils::{managed_buffer_utils::ManagedBufferUtils, managed_vec_utils::EqUtils},
};
use core::ops::Deref;
use elrond_wasm::{elrond_codec::TopEncode, formatter::SCDisplay};

use super::{item::Item, slot::Slot};

pub const ERR_NAME_CONTAINS_UNSUPPORTED_CHARACTERS: &str =
    "A name can't contains colon or semicolons";

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(ManagedVecItem, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone, Debug)]
struct EquippableNftAttribute<M: ManagedTypeApi> {
    pub slot: Slot<M>,
    pub name: Option<ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> EquippableNftAttribute<M> {
    pub fn to_kvp_buffer(&self) -> ManagedBuffer<M> {
        let item_name = match &self.name {
            Some(item) => item.clone(),
            None => ManagedBuffer::new_from_bytes(UNEQUIPPED_ITEM_NAME),
        };

        let item = &Item {
            name: item_name,
            slot: self.slot.clone(),
        };

        let mut output_buffer = ManagedBuffer::<M>::new();
        let mut item_buffer = ManagedBuffer::new();

        item.top_encode(&mut item_buffer).unwrap();
        output_buffer.append(&item_buffer);

        return output_buffer;
    }
}

#[derive(NestedEncode, NestedDecode, TypeAbi, Debug, Clone)]
pub struct EquippableNftAttributes<M: ManagedTypeApi> {
    items: ManagedVec<M, EquippableNftAttribute<M>>,
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
        return self.items.eq_unorder(&other.items);
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

            if &item.name == &ManagedBuffer::new_from_bytes(UNEQUIPPED_ITEM_NAME) {
                equippable_attributes.set_item_if_empty(&slot, None);
                continue;
            } else {
                equippable_attributes.set_item_if_empty(&slot, Some(item.name));
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

        for (i, kvp) in self.items.iter().enumerate() {
            managed_buffer.append(&kvp.to_kvp_buffer());

            // add comma, except for the last line
            if i < self.items.len() - 1 {
                managed_buffer.append_bytes(b";");
            }
        }

        let bytes = managed_buffer.load_512_bytes();

        output.set_slice_u8(&bytes[..managed_buffer.len()]);

        return Result::Ok(());
    }
}

impl<M: ManagedTypeApi> EquippableNftAttributes<M> {
    pub fn empty() -> Self {
        return EquippableNftAttributes {
            items: ManagedVec::new(),
        };
    }

    pub fn get_item(&self, slot: &Slot<M>) -> Option<Item<M>> {
        if let Some(index) = self.__get_index(&slot) {
            if let Some(name) = self.items.get(index).name {
                return Option::<Item<M>>::Some(Item {
                    slot: slot.clone(),
                    name,
                });
            }
        }

        return Option::None;
    }

    pub fn set_item_if_empty(&mut self, slot: &Slot<M>, name: Option<ManagedBuffer<M>>) {
        if self.is_slot_empty(slot) == false {
            sc_panic_self!(
                M,
                "The slot is not empty. Please free it, before setting an item."
            );
        }

        return self.set_item(slot, name);
    }

    pub fn set_item(&mut self, slot: &Slot<M>, opt_name: Option<ManagedBuffer<M>>) {
        let index = self.__get_index(&slot);

        panic_if_name_contains_unsupported_characters::<M>(&opt_name);

        let new_equippable_attribute = EquippableNftAttribute {
            slot: slot.clone(),
            name: opt_name,
        };

        match index {
            Some(index) => {
                let result = self.items.set(index, &new_equippable_attribute);

                if result.is_err() {
                    sc_panic_self!(
                        M,
                        "Failed to set item, InvalidSliceError exception happened."
                    );
                }
            }
            None => {
                self.items.push(new_equippable_attribute);
            }
        }

        self.items = self.items.sort_alphabetically();
    }

    pub fn is_slot_empty(&self, slot: &Slot<M>) -> bool {
        match self.get_item(slot) {
            Some(_) => false,
            None => true,
        }
    }

    pub fn empty_slot(&mut self, slot: &Slot<M>) {
        return self.set_item(&slot, Option::None);
    }

    fn __get_index(&self, slot: &Slot<M>) -> Option<usize> {
        return self.items.iter().position(|kvp| &kvp.slot == slot);
    }
}

fn panic_if_name_contains_unsupported_characters<M: ManagedTypeApi>(
    opt_name: &Option<ManagedBuffer<M>>,
) {
    if let Some(name) = opt_name.clone() {
        if name.contains(b";") || name.contains(b":") {
            sc_panic_self!(M, ERR_NAME_CONTAINS_UNSUPPORTED_CHARACTERS);
        }
    }
}
