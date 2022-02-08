#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use crate::item::Item;
use crate::item_slot::ItemSlot;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
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
        match self.is_slot_empty(slot) {
            Result::Ok(false) => {
                return Result::Err(ManagedBuffer::new_from_bytes(
                    b"The slot is not empty. Please free it, before setting an item.",
                ))
            }
            Result::Err(()) => {
                return Result::Err(ManagedBuffer::new_from_bytes(
                    b"Error while getting slot is empty",
                ))
            }
            _ => {}
        }

        return self.__set_item_no_check(slot, item);
    }

    #[allow(unreachable_patterns)]
    pub fn get_item(&self, slot: &ItemSlot) -> Result<Option<Item<M>>, ()> {
        match slot {
            &ItemSlot::Hat => return Result::Ok(self.hat.clone()),
            &ItemSlot::Background => return Result::Ok(self.background.clone()),
            &ItemSlot::Skin => return Result::Ok(self.skin.clone()),
            &ItemSlot::Chain => return Result::Ok(self.chain.clone()),
            &ItemSlot::Beak => return Result::Ok(self.beak.clone()),
            &ItemSlot::Weapon => return Result::Ok(self.weapon.clone()),
            &ItemSlot::Clothes => return Result::Ok(self.clothes.clone()),
            &ItemSlot::Eye => return Result::Ok(self.eye.clone()),
            _ => return Result::Err(()),
        };
    }

    #[allow(unreachable_patterns)]
    pub fn is_slot_empty(&self, slot: &ItemSlot) -> Result<bool, ()> {
        let item = self.get_item(slot);

        match item {
            Result::Ok(Some(_)) => Result::Ok(false),
            Result::Ok(None) => Result::Ok(true),
            Result::Err(_) => Result::Err(()),
        }
    }

    pub fn empty_slot(&mut self, slot: &ItemSlot) -> Result<(), ManagedBuffer<M>> {
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
            _ => {
                return Result::Err(ManagedBuffer::new_from_bytes(
                    b"The slot provided is not supported",
                ))
            }
        }

        Result::Ok(())
    }
}
