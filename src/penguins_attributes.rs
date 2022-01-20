use crate::item_slot::ItemSlot;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct PenguinAttributes<M: ManagedTypeApi> {
    pub hat: (TokenIdentifier<M>, u64), // pub background: TokenIdentifier<M>,
}

impl<M: ManagedTypeApi> PenguinAttributes<M> {
    pub fn set_item(
        &mut self,
        slot: &ItemSlot,
        token: TokenIdentifier<M>,
        nonce: u64,
    ) -> Result<(), ManagedBuffer<M>> {
        if token != self.empty_item() {
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
            _ => return Result::Err(()),
        };
    }

    #[allow(unreachable_patterns)]
    pub fn is_slot_empty(&self, slot: &ItemSlot) -> Result<bool, ()> {
        match slot {
            &ItemSlot::Hat => return Result::Ok(self.hat.0.is_empty()),
            _ => return Result::Err(()),
        };
    }

    pub fn empty_slot(&mut self, slot: &ItemSlot) -> Result<(), ManagedBuffer<M>> {
        return self.set_item(slot, self.empty_item(), 0);
    }

    pub fn empty_item(&self) -> TokenIdentifier<M> {
        return TokenIdentifier::<M>::from(ManagedBuffer::<M>::new());
    }
}
