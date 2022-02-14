#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use crate::item::Item;
use crate::item_slot::ItemSlot;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Debug)]
pub struct PenguinAttributes<M: ManagedTypeApi> {
    pub hat: Option<Item<M>>,
    pub background: Option<Item<M>>,
    pub skin: Option<Item<M>>,
    pub chain: Option<Item<M>>,
    pub beak: Option<Item<M>>,
    pub weapon: Option<Item<M>>,
    pub clothes: Option<Item<M>>,
    pub eye: Option<Item<M>>,
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

    #[allow(unreachable_patterns)]
    pub fn get_item(&self, slot: &ItemSlot) -> Option<Item<M>> {
        match slot {
            &ItemSlot::Hat => return self.hat.clone(),
            &ItemSlot::Background => return self.background.clone(),
            &ItemSlot::Skin => return self.skin.clone(),
            &ItemSlot::Chain => return self.chain.clone(),
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
            chain: Option::None,
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
            ItemSlot::Chain => self.chain = item,
            ItemSlot::Beak => self.beak = item,
            ItemSlot::Weapon => self.weapon = item,
            ItemSlot::Clothes => self.clothes = item,
            ItemSlot::Eye => self.eye = item,
            _ => panic!("Missing slot. Please add it in get_item"),
        }

        Result::Ok(())
    }
}
