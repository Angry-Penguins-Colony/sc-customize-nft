#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use elrond_wasm::imports;
use elrond_wasm::String;

imports!();

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PenguinAttributes<M: ManagedTypeApi> {
    pub hat: ManagedBuffer<M>,
    pub background: ManagedBuffer<M>,
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
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();

        // reads attributes from the penguin
        // should => This only works for addresses that are in the same shard as the smart contract.
        // how to send NFT and call a method ?
        self.blockchain()
            .get_esdt_token_data(&caller, &penguin_id, penguin_nonce);

        // self.blockchain().
        //     .decode_attributes::<YourStruct<Self::Api>>()?;

        // for item_id in items_ids {
        //     // determine itemType from ID
        //     let item_type_out = self.get_item_type(&item_id);

        //     if let OptionalResult::None = item_type_out {
        //         require!(false, "An items provided is not considered like an item.")
        //     }

        //     // set attributes[itemType] = item_id

        //     // burn player item
        // }

        // update penguin

        Ok(())
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
