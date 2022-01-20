#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod item_attributes;
pub mod item_slot;
pub mod penguins_attributes;

use item_attributes::ItemAttributes;
use item_slot::ItemSlot;
use penguins_attributes::PenguinAttributes;

#[elrond_wasm::derive::contract]
pub trait Equip {
    #[storage_mapper("items_types")]
    fn items_types(&self) -> MapMapper<ItemSlot, ManagedVec<TokenIdentifier>>;

    #[storage_mapper("penguins_identifier")]
    fn penguins_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[init]
    fn init(&self, penguins_identifier: TokenIdentifier) -> SCResult<()> {
        self.penguins_identifier().set(&penguins_identifier);

        return Ok(());
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
        require!(
            penguin_id == &self.penguins_identifier().get(),
            "Please provide a penguin"
        );

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
                            let result = self.sent_item_from_slot(&mut attributes, &item_slot);

                            match result {
                                SCResult::Err(err) => return SCResult::Err(err),
                                _ => (),
                            }
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
        let mut attributes = self.parse_penguin_attributes(penguin_id, penguin_nonce);

        for slot in slots {
            let result = self.sent_item_from_slot(&mut attributes, &slot);

            match result {
                SCResult::Err(err) => return SCResult::Err(err),
                _ => (),
            }
        }

        // return items nonces
        return self.update_penguin(&penguin_id, penguin_nonce, &attributes);
    }

    /// Empty the item at the slot proivided and sent it to the caller.
    fn sent_item_from_slot(
        &self,
        attributes: &mut PenguinAttributes<Self::Api>,
        slot: &ItemSlot,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();

        require!(
            attributes.is_slot_empty(&slot).unwrap() == false,
            "Cannot sent item from an empty slot"
        );

        let result = attributes.get_item(&slot);

        if let Result::Err(()) = result {
            return SCResult::Err("Error while minting and sending an item".into());
        }

        let (item_id, item_nonce) = result.unwrap().into_tuple();

        self.send()
            .esdt_local_mint(&item_id, item_nonce, &BigUint::from(1u32));

        self.send()
            .direct(&caller, &item_id, item_nonce, &BigUint::from(1u32), &[]);

        attributes.empty_slot(&slot);

        return SCResult::Ok(());
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
