#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use core::ops::Deref;

use alloc::{borrow::ToOwned, format};
use elrond_wasm::elrond_codec::{TopDecodeInput, TopEncode};

use super::item::Item;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Debug)]
pub struct PenguinAttributes<M: ManagedTypeApi> {
    // TODO: find a structure that mimic a map
    // Is there a ManagedSet or ManagedMap ? no.
    // itemsBySlots: ManagedVec<M, Item<M>>,
    itemsBySlots: ManagedVec<M, ManagedBuffer<M>>,
    itemsX: ManagedVec<M, Item<M>>,
}

// impl<M: ManagedTypeApi> TopDecode for PenguinAttributes<M> {
//     fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
//         let unequipped_buffer = ManagedBuffer::<M>::new_from_bytes(b"unequipped");

//         let mut penguin = PenguinAttributes::empty();

//         let buffer = <ManagedBuffer<M> as TopDecode>::top_decode(input).unwrap();
//         let items_raw = split_buffer(&buffer, b';');

//         for item_raw in items_raw.iter() {
//             let parts = split_buffer(item_raw.deref(), b':');

//             let slot_buffer = parts.get(0).deref().to_owned();
//             let item_buffer = parts.get(1);

//             let item = if item_buffer.deref() == &unequipped_buffer {
//                 None
//             } else {
//                 Some(Item::top_decode(item_buffer.deref()).unwrap())
//             };

//             let slot = ItemSlot::from(slot_buffer);

//             if slot == ItemSlot::None {
//                 return Result::Err(DecodeError::from(&b"Unable to parse a slot"[..]));
//             }

//             let _ = penguin.set_item(&slot, item);
//         }

//         return Result::Ok(penguin);
//     }
// }

// impl<M: ManagedTypeApi> TopEncode for PenguinAttributes<M> {
//     fn top_encode<O: elrond_codec::TopEncodeOutput>(
//         &self,
//         output: O,
//     ) -> Result<(), elrond_codec::EncodeError> {
//         let mut managed_buffer = ManagedBuffer::<M>::new();

//         for (i, slot) in ItemSlot::VALUES.iter().enumerate() {
//             managed_buffer.append(&self.to_managed_buffer(slot));

//             // add comma, except for the last line
//             if i < ItemSlot::VALUES.len() - 1 {
//                 managed_buffer.append_bytes(b";");
//             }
//         }

//         let mut bytes: [u8; 512] = [0; 512];
//         managed_buffer.load_to_byte_array(&mut bytes);
//         output.set_slice_u8(&bytes[..managed_buffer.len()]);

//         return Result::Ok(());
//     }
// }

impl<M: ManagedTypeApi> PenguinAttributes<M> {
    pub fn new(items_by_slot: &[(&ManagedBuffer<M>, Item<M>)]) -> Self {
        let mut attributes = Self::empty();

        for (slot, item) in items_by_slot {
            let result = attributes.set_item(slot, Option::Some(item.clone()));

            if result.is_err() {
                panic!("Failed to set item on slot");
            }
        }

        return attributes;
    }

    #[allow(unreachable_patterns)]
    pub fn get_item(&self, slot: &ManagedBuffer<M>) -> Option<Item<M>> {
        panic!("Not implemented");
    }

    pub fn set_item(
        &mut self,
        slot: &ManagedBuffer<M>,
        item: Option<Item<M>>,
    ) -> Result<(), ManagedBuffer<M>> {
        panic!("Not implemented");
        // if self.is_slot_empty(slot) == false {
        //     return Result::Err(ManagedBuffer::new_from_bytes(
        //         b"The slot is not empty. Please free it, before setting an item.",
        //     ));
        // }

        // return self.__set_item_no_check(slot, item);
    }

    pub fn get_count(&self) -> usize {
        panic!("Not implemented");
    }

    #[allow(unreachable_patterns)]
    pub fn is_slot_empty(&self, slot: &ManagedBuffer<M>) -> bool {
        let item = self.get_item(slot);

        match item {
            Some(_) => false,
            None => true,
        }
    }

    pub fn empty_slot(&mut self, slot: &ManagedBuffer<M>) -> Result<(), ManagedBuffer<M>> {
        panic!("Not implemented");
        // return self.__set_item_no_check(slot, Option::None);
    }

    pub fn empty() -> Self {
        return PenguinAttributes {
            itemsBySlots: ManagedVec::new(),
            itemsX: ManagedVec::new(),
        };
    }

    /// Set an item on a slot, without checking if the slot is empty.
    fn __set_item_no_check(
        &mut self,
        slot: &ManagedBuffer<M>,
        item: Option<Item<M>>,
    ) -> Result<(), ManagedBuffer<M>> {
        panic!("Not implemented");

        // #[allow(unreachable_patterns)]
        // match slot {
        //     ItemSlot::Hat => self.hat = item,
        //     ItemSlot::Background => self.background = item,
        //     ItemSlot::Skin => self.skin = item,
        //     ItemSlot::Beak => self.beak = item,
        //     ItemSlot::Weapon => self.weapon = item,
        //     ItemSlot::Clothes => self.clothes = item,
        //     ItemSlot::Eye => self.eye = item,
        //     _ => panic!("Missing slot. Please add it in get_item"),
        // }
    }

    fn to_managed_buffer(&self, slot: &ManagedBuffer<M>) -> ManagedBuffer<M> {
        panic!("Not implemented");

        // let item = match self.get_item(slot) {
        //     Some(item) => {
        //         let mut output = ManagedBuffer::new();

        //         // TODO: optimize Item.top_encode
        //         // TODO: we unwrap to trigger if top_encode fails, but there must be a better way
        //         item.top_encode(&mut output).unwrap();

        //         output
        //     }
        //     None => ManagedBuffer::<M>::new_from_bytes(b"unequipped"),
        // };

        // let mut managed_buffer = ManagedBuffer::<M>::new();
        // managed_buffer.append_bytes(slot.to_title_bytes::<M>());
        // managed_buffer.append_bytes(b":");
        // managed_buffer.append(&item);

        // return managed_buffer;
    }
}
