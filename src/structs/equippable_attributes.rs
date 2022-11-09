use crate::{
    constants::UNEQUIPPED_ITEM_NAME,
    utils::{managed_buffer_utils::ManagedBufferUtils, managed_vec_utils::EqUtils},
};
use core::ops::Deref;
use elrond_wasm::{elrond_codec::TopEncode, formatter::SCDisplay};

pub const ERR_NAME_CONTAINS_UNSUPPORTED_CHARACTERS: &[u8] =
    b"A name can't contain colon or semicolons";
pub const ERR_SLOT_CONTAINS_UNSUPPORTED_CHARACTERS: &[u8] =
    b"A slot can't containscolon or semicolons";

pub const ERR_NAME_CANNOT_BE_UNEQUIPPED: &[u8] = b"The name cannot be 'unequipped'.";

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(ManagedVecItem, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone, Debug)]
struct EquippableAttribute<M: ManagedTypeApi> {
    pub slot: ManagedBuffer<M>,
    pub name: Option<ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> EquippableAttribute<M> {
    pub fn to_buffer(&self) -> ManagedBuffer<M> {
        let item_name = match &self.name {
            Some(name) => name.clone(),
            None => ManagedBuffer::new_from_bytes(UNEQUIPPED_ITEM_NAME),
        };

        let mut output = ManagedBuffer::<M>::new();

        // build buffer
        output.append(&self.slot);
        output.append_bytes(b":");
        output.append(&item_name);

        return output;
    }

    pub fn from_buffer(input: ManagedBuffer<M>) -> Self {
        let parts = input.split(b':');

        if parts.len() != 2 {
            M::error_api_impl().signal_error(b"cannot decode EquippableNftAttribute");
        }

        let name = parts.get(1).deref().clone();

        let opt_name = if name == ManagedBuffer::<M>::new_from_bytes(UNEQUIPPED_ITEM_NAME) {
            None
        } else {
            Some(name)
        };

        return Self {
            slot: parts.get(0).deref().clone(),
            name: opt_name,
        };
    }
}

#[derive(NestedEncode, NestedDecode, TypeAbi, Debug, Clone)]
pub struct EquippableAttributes<M: ManagedTypeApi> {
    items: ManagedVec<M, EquippableAttribute<M>>,
}

pub trait SortUtils<M>
where
    M: ManagedTypeApi,
{
    fn sort_alphabetically(&self) -> Self;
}
impl<M> SortUtils<M> for ManagedVec<M, EquippableAttribute<M>>
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

impl<M: ManagedTypeApi + core::cmp::PartialEq> PartialEq for EquippableAttributes<M> {
    fn eq(&self, other: &Self) -> bool {
        return self.items.eq_unorder(&other.items);
    }
}

impl<M: ManagedTypeApi> SCDisplay for EquippableAttributes<M> {
    fn fmt<F: elrond_wasm::formatter::FormatByteReceiver>(&self, f: &mut F) {
        let mut output = ManagedBuffer::<F::Api>::new_from_bytes(b"");

        let _ = self.top_encode(&mut output);
        f.append_managed_buffer(&output);
    }
}

impl<M: ManagedTypeApi> TopDecode for EquippableAttributes<M> {
    fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let mut equippable_attributes = EquippableAttributes::empty();

        let buffer = <ManagedBuffer<M> as TopDecode>::top_decode(input).unwrap();
        let items_raw = buffer.split(b';');

        for item_raw in items_raw.iter() {
            let attribute = EquippableAttribute::from_buffer(item_raw.deref().clone());

            equippable_attributes.set_item_if_empty(&attribute.slot, attribute.name);
        }

        return Result::Ok(equippable_attributes);
    }
}

impl<M: ManagedTypeApi> TopEncode for EquippableAttributes<M> {
    fn top_encode<O: elrond_codec::TopEncodeOutput>(
        &self,
        output: O,
    ) -> Result<(), elrond_codec::EncodeError> {
        let mut managed_buffer = ManagedBuffer::<M>::new();

        for (i, kvp) in self.items.iter().enumerate() {
            managed_buffer.append(&kvp.to_buffer());

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

impl<M: ManagedTypeApi> EquippableAttributes<M> {
    pub fn empty() -> Self {
        return EquippableAttributes {
            items: ManagedVec::new(),
        };
    }

    pub fn get_name(&self, slot: &ManagedBuffer<M>) -> Option<ManagedBuffer<M>> {
        if let Some(index) = self.get_index(&slot) {
            return self.items.get(index).name;
        } else {
            return Option::None;
        }
    }

    pub fn set_item_if_empty(&mut self, slot: &ManagedBuffer<M>, name: Option<ManagedBuffer<M>>) {
        if self.is_slot_empty(slot) == false {
            M::error_api_impl()
                .signal_error(b"The slot is not empty. Please free it, before setting an item.");
        }

        return self.set_item(slot, name);
    }

    pub fn set_item(&mut self, slot: &ManagedBuffer<M>, opt_name: Option<ManagedBuffer<M>>) {
        let index = self.get_index(&slot);

        panic_if_name_contains_unsupported_characters(&opt_name);
        panic_if_slot_contains_unsupported_characters(slot);

        let new_equippable_attribute = EquippableAttribute {
            slot: slot.clone(),
            name: opt_name,
        };

        match index {
            Some(index) => {
                let result = self.items.set(index, &new_equippable_attribute);

                if result.is_err() {
                    M::error_api_impl()
                        .signal_error(b"Failed to set item, InvalidSliceError exception happened.");
                }
            }
            None => {
                self.items.push(new_equippable_attribute);
            }
        }

        self.items = self.items.sort_alphabetically();
    }

    pub fn is_slot_empty(&self, slot: &ManagedBuffer<M>) -> bool {
        match self.get_name(slot) {
            Some(_) => false,
            None => true,
        }
    }

    pub fn empty_slot(&mut self, slot: &ManagedBuffer<M>) {
        return self.set_item(&slot, Option::None);
    }

    fn get_index(&self, slot: &ManagedBuffer<M>) -> Option<usize> {
        return self.items.iter().position(|kvp| &kvp.slot == slot);
    }
}

pub fn panic_if_name_contains_unsupported_characters<M: ManagedTypeApi>(
    opt_name: &Option<ManagedBuffer<M>>,
) {
    if let Some(name) = opt_name.clone() {
        if name.contains_char(b';') || name.contains_char(b':') {
            M::error_api_impl().signal_error(ERR_NAME_CONTAINS_UNSUPPORTED_CHARACTERS);
        }

        if name == ManagedBuffer::new_from_bytes(UNEQUIPPED_ITEM_NAME) {
            M::error_api_impl().signal_error(ERR_NAME_CANNOT_BE_UNEQUIPPED);
        }
    }
}

pub fn panic_if_slot_contains_unsupported_characters<M: ManagedTypeApi>(slot: &ManagedBuffer<M>) {
    if slot.contains_char(b';') || slot.contains_char(b':') {
        M::error_api_impl().signal_error(ERR_SLOT_CONTAINS_UNSUPPORTED_CHARACTERS);
    }
}
