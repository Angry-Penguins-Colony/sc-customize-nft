#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

// use elrond_wasm::elrond_codec::TopEncode;
// use elrond_wasm::imports;
// use elrond_wasm::String;

// imports!();

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Debug, PartialEq)]
pub enum ItemSlot {
    Hat,
}

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
    ) -> Result<(), ()> {
        // match self.is_slot_empty(slot) {
        //     Result::Ok(false) => return Result::Err(()),
        //     Result::Err(()) => return Result::Err(()),
        //     _ => {}
        // }

        match slot {
            ItemSlot::Hat => self.hat = (token, nonce),
            _ => return Result::Err(()),
        }

        Result::Ok(())
    }

    pub fn get_item(&self, slot: &ItemSlot) -> Result<MultiResult2<TokenIdentifier<M>, u64>, ()> {
        match slot {
            &ItemSlot::Hat => return Result::Ok(MultiResult2::from(self.hat.clone())),
            _ => return Result::Err(()),
        };
    }

    pub fn is_slot_empty(&self, slot: &ItemSlot) -> Result<bool, ()> {
        match slot {
            &ItemSlot::Hat => return Result::Ok(self.hat.0.is_empty()),
            _ => return Result::Err(()),
        };
    }

    pub fn empty_slot(&mut self, slot: &ItemSlot) -> Result<(), ()> {
        return self.set_item(slot, self.empty_item(), 0);
    }

    pub fn empty_item(&self) -> TokenIdentifier<M> {
        return TokenIdentifier::<M>::from(ManagedBuffer::<M>::new());
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct ItemAttributes {}

#[elrond_wasm::derive::contract]
pub trait Equip {
    #[storage_mapper("items_types")]
    fn items_types(&self) -> MapMapper<ItemSlot, ManagedVec<TokenIdentifier>>;

    #[init]
    fn init(&self) -> SCResult<()> {
        Ok(())
    }

    #[endpoint(registerItem)]
    #[only_owner]
    fn register_item(
        &self,
        item_type: ItemSlot,
        #[var_args] items_id: ManagedVarArgs<TokenIdentifier>,
    ) -> SCResult<()> {
        // TODO tester si ça override pas
        self.items_types().insert(item_type, items_id.to_vec());

        Ok(())
    }

    #[view(getItemType)]
    fn get_item_type(&self, item_id: &TokenIdentifier) -> OptionalResult<ItemSlot> {
        // iterate over all items_types
        for (item_type, compare_items_ids) in self.items_types().iter() {
            for compare_item_id in compare_items_ids.iter() {
                if &compare_item_id == item_id {
                    return OptionalResult::Some(item_type);
                }
            }
        }

        return OptionalResult::None;
    }

    #[endpoint]
    fn equip(
        &self,
        penguin_id: &TokenIdentifier,
        penguin_nonce: u64,
        #[var_args] items_token: ManagedVarArgs<MultiArg2<TokenIdentifier, u64>>,
    ) -> SCResult<u64> {
        let mut attributes = self.parse_penguin_attributes(penguin_id, penguin_nonce);

        // let's equip each item
        for item_token in items_token {
            let (item_id, item_nonce) = item_token.into_tuple();

            // determine itemType from ID
            let item_slot_out = self.get_item_type(&item_id);

            match item_slot_out {
                OptionalResult::Some(item_slot) => {
                    match attributes.is_slot_empty(&item_slot) {
                        Result::Ok(false) => {
                            // slot is not empty, we need to remove it

                            let (item_id, item_nonce) =
                                attributes.get_item(&item_slot).unwrap().into_tuple();

                            self.send()
                                .esdt_local_mint(&item_id, item_nonce, &BigUint::from(1u32));

                            self.send().direct(
                                &self.blockchain().get_caller(),
                                &item_id,
                                item_nonce,
                                &BigUint::from(1u32),
                                &[],
                            );

                            attributes.empty_slot(&item_slot);
                        }
                        Result::Err(_) => {
                            require!(false, "Error while checking if slot is empty");
                        }
                        _ => {}
                    }

                    let result = attributes.set_item(&item_slot, item_id.clone(), item_nonce);
                    require!(
                        result == Result::Ok(()),
                        "Cannot set item. Maybe the item is not considered like an item."
                    );
                }
                OptionalResult::None => {
                    require!(false, "An items provided is not considered like an item.")
                }
            }

            self.send()
                .esdt_local_burn(&item_id, item_nonce, &BigUint::from(1u32));
        }

        // update penguin
        return self.update_penguin(&penguin_id, penguin_nonce, &attributes);
    }

    #[endpoint]
    fn desequip(
        &self,
        penguin_id: &TokenIdentifier,
        penguin_nonce: u64,
        #[var_args] slots: ManagedVarArgs<ItemSlot>,
    ) -> SCResult<u64> {
        let caller = self.blockchain().get_caller();
        let mut attributes = self.parse_penguin_attributes(penguin_id, penguin_nonce);

        for slot in slots {
            let result = attributes.get_item(&slot);

            if let Result::Err(()) = result {
                return SCResult::Err("Error while getting an item".into());
            }

            let (item_id, item_nonce) = result.unwrap().into_tuple();

            self.send()
                .esdt_local_mint(&item_id, item_nonce, &BigUint::from(1u32));

            self.send()
                .direct(&caller, &item_id, item_nonce, &BigUint::from(1u32), &[]);

            attributes.empty_slot(&slot);
        }

        // return items nonces
        return self.update_penguin(&penguin_id, penguin_nonce, &attributes);
    }

    fn parse_penguin_attributes(
        &self,
        penguin_id: &TokenIdentifier,
        penguin_nonce: u64,
    ) -> PenguinAttributes<Self::Api> {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(&self.blockchain().get_caller(), &penguin_id, penguin_nonce)
            .decode_attributes::<PenguinAttributes<Self::Api>>()
            .unwrap();
        return attributes;
    }

    fn update_penguin(
        &self,
        penguin_id: &TokenIdentifier,
        penguin_nonce: u64,
        attributes: &PenguinAttributes<Self::Api>,
    ) -> SCResult<u64> {
        let caller = self.blockchain().get_caller();

        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new_from_bytes(b"https://www.google.com"));

        // let mut serialized_attributes = Vec::new();
        // &new_attributes.top_encode(&mut serialized_attributes)?;

        // let attributes_hash = self.crypto().sha256(&serialized_attributes);
        // let hash_buffer = ManagedBuffer::from(attributes_hash.as_bytes());

        // self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(

        self.send()
            .esdt_local_burn(&penguin_id, penguin_nonce, &BigUint::from(1u32));

        let token_nonce = self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(
            &penguin_id,
            &BigUint::from(1u32),
            &ManagedBuffer::new_from_bytes(b"new penguin"),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &attributes,
            &uris,
        );

        self.send()
            .direct(&caller, &penguin_id, token_nonce, &BigUint::from(1u32), &[]);

        return Ok(token_nonce);
    }
}
