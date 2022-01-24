#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use crate::item_slot::ItemSlot;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct PenguinAttributes<M: ManagedTypeApi> {
    pub hat: (TokenIdentifier<M>, u64),
    pub background: (TokenIdentifier<M>, u64),
    pub skin: (TokenIdentifier<M>, u64),
    pub chain: (TokenIdentifier<M>, u64),
    pub beak: (TokenIdentifier<M>, u64),
    pub weapon: (TokenIdentifier<M>, u64),
    pub clothes: (TokenIdentifier<M>, u64),
    pub eye: (TokenIdentifier<M>, u64),
}

impl<M: ManagedTypeApi> Default for PenguinAttributes<M> {
    fn default() -> Self {
        let empty_item = (TokenIdentifier::<M>::from_esdt_bytes(b""), 0);

        Self {
            hat: empty_item.clone(),
            background: empty_item.clone(),
            skin: empty_item.clone(),
            chain: empty_item.clone(),
            beak: empty_item.clone(),
            weapon: empty_item.clone(),
            clothes: empty_item.clone(),
            eye: empty_item.clone(),
        }
    }
}

impl<M: ManagedTypeApi> PenguinAttributes<M> {
    pub fn set_item(
        &mut self,
        slot: &ItemSlot,
        token: TokenIdentifier<M>,
        nonce: u64,
    ) -> Result<(), ManagedBuffer<M>> {
        if token != self.get_empty_item() {
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
        }

        #[allow(unreachable_patterns)]
        match slot {
            ItemSlot::Hat => self.hat = (token, nonce),
            ItemSlot::Background => self.background = (token, nonce),
            ItemSlot::Skin => self.skin = (token, nonce),
            ItemSlot::Chain => self.chain = (token, nonce),
            ItemSlot::Beak => self.beak = (token, nonce),
            ItemSlot::Weapon => self.weapon = (token, nonce),
            ItemSlot::Clothes => self.clothes = (token, nonce),
            ItemSlot::Eye => self.eye = (token, nonce),
            _ => {
                return Result::Err(ManagedBuffer::new_from_bytes(
                    b"The slot provided is not supported",
                ))
            }
        }

        Result::Ok(())
    }

    #[allow(unreachable_patterns)]
    pub fn get_item(&self, slot: &ItemSlot) -> Result<MultiResult2<TokenIdentifier<M>, u64>, ()> {
        match slot {
            &ItemSlot::Hat => return Result::Ok(MultiResult2::from(self.hat.clone())),
            &ItemSlot::Background => {
                return Result::Ok(MultiResult2::from(self.background.clone()))
            }
            &ItemSlot::Skin => return Result::Ok(MultiResult2::from(self.skin.clone())),
            &ItemSlot::Chain => return Result::Ok(MultiResult2::from(self.chain.clone())),
            &ItemSlot::Beak => return Result::Ok(MultiResult2::from(self.beak.clone())),
            &ItemSlot::Weapon => return Result::Ok(MultiResult2::from(self.weapon.clone())),
            &ItemSlot::Clothes => return Result::Ok(MultiResult2::from(self.clothes.clone())),
            &ItemSlot::Eye => return Result::Ok(MultiResult2::from(self.eye.clone())),
            _ => return Result::Err(()),
        };
    }

    #[allow(unreachable_patterns)]
    pub fn is_slot_empty(&self, slot: &ItemSlot) -> Result<bool, ()> {
        return Result::Ok(self.get_item(slot)?.into_tuple().0.is_empty());
    }

    pub fn empty_slot(&mut self, slot: &ItemSlot) -> Result<(), ManagedBuffer<M>> {
        return self.set_item(slot, self.get_empty_item(), 0);
    }

    fn get_empty_item(&self) -> TokenIdentifier<M> {
        return TokenIdentifier::<M>::from(ManagedBuffer::<M>::new());
    }
}
