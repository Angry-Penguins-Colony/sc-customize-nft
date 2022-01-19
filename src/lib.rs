#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use elrond_wasm::elrond_codec::TopEncode;
use elrond_wasm::imports;
use elrond_wasm::String;

imports!();

elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct PenguinAttributes<M: ManagedTypeApi> {
    pub hat: TokenIdentifier<M>,
    // pub background: TokenIdentifier<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct ItemAttributes {}

#[elrond_wasm::derive::contract]
pub trait Equip {
    #[storage_mapper("items_types")]
    fn items_types(&self) -> MapMapper<ManagedBuffer, ManagedVec<TokenIdentifier>>;

    #[init]
    fn init(&self) -> SCResult<()> {
        Ok(())
    }

    #[endpoint(registerItem)]
    #[only_owner]
    fn register_item(
        &self,
        item_type: &ManagedBuffer,
        #[var_args] items_id: ManagedVarArgs<TokenIdentifier>,
    ) -> SCResult<()> {
        // TODO tester si ça override pas
        self.items_types()
            .insert(item_type.clone(), items_id.to_vec());

        Ok(())
    }

    #[view(getItemType)]
    fn get_item_type(&self, item_id: &TokenIdentifier) -> OptionalResult<ManagedBuffer> {
        // iterate over all items_types
        for (item_type, compare_items_ids) in self.items_types().iter() {
            for compare_item_id in compare_items_ids.iter() {
                if &compare_item_id == item_id {
                    return OptionalResult::Some(item_type.clone());
                }
            }
        }

        return OptionalResult::None;
    }

    // fn update_penguin(&self, penguin_id: &TokenIdentifier, item_id: &TokenIdentifier) {
    //     // TODO
    // }

    #[endpoint]
    fn equip(
        &self,
        penguin_id: &TokenIdentifier,
        penguin_nonce: u64,
        #[var_args] items_ids: ManagedVarArgs<TokenIdentifier>,
    ) -> SCResult<u64> {
        let caller = self.blockchain().get_caller();

        let mut attributes = self
            .blockchain()
            .get_esdt_token_data(&caller, &penguin_id, penguin_nonce)
            .decode_attributes::<PenguinAttributes<Self::Api>>()
            .unwrap();

        for item_id in items_ids {
            // determine itemType from ID
            let item_type_out = self.get_item_type(&item_id);

            if let OptionalResult::None = item_type_out {
                require!(false, "An items provided is not considered like an item.")
            }

            // if (OptionalResult::Some(item_type_out)) = ManagedBuffer::new_from_bytes(b"hat") {
            //     require!(false, "A penguin can't have more than one hat.")
            // }

            let _hat = ManagedBuffer::new_from_bytes(b"hat");
            let _bg = ManagedBuffer::new_from_bytes(b"background");

            match Some(item_type_out) {
                Some(_hat) => attributes.hat = item_id,
                // Some(_bg) => attributes.background = item_id,
                _ => require!(false, "An items provided is not considered like an item."),
            }

            // TODO: mint previous item if there were one
            // TODO: burn player item
        }

        // update penguin
        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new_from_bytes(b"https://www.google.com"));

        // let mut serialized_attributes = Vec::new();
        // &new_attributes.top_encode(&mut serialized_attributes)?;

        // let attributes_hash = self.crypto().sha256(&serialized_attributes);
        // let hash_buffer = ManagedBuffer::from(attributes_hash.as_bytes());

        // self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(

        // burn the old one
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

        Ok(token_nonce)
    }

    // #[endpoint]
    // fn equip(&self, penguin_id: &String, items_ids: &[String]) -> SCResult<()> {
    //     for item_id in items_ids {

    //         // determine itemType from ID

    //         // set attributes[itemType] = item_id

    //         // burn player item

    //         // update penguin attributes
    //     }

    //     Ok(())
    // }
}
