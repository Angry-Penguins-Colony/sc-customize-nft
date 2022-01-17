#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use elrond_wasm::imports;
use elrond_wasm::String;

imports!();

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
    fn register_item(&self, item_type: &ManagedBuffer, #[var_args] items_id: ManagedVarArgs<TokenIdentifier>)
    {
        // TODO tester si ça override pas
        self.items_types().insert(item_type.clone(), items_id.to_vec());
    }


    // #[endpoint]
    // fn equip(&self, penguin_id: &String, items_ids: &[String]) -> SCResult<()> {
    //     for item_id in items_ids {
    //
    //         // determine itemType from ID
    //
    //         // set attributes[itemType] = item_id
    //
    //         // burn player item
    //
    //         // update penguin attributes
    //     }
    //
    //     Ok(())
    // }

    // #[view(getItemType)]
    // fn get_item_type(&self, item_id: &String) -> OptionalResult<String> {
    //     for (item_type, items_ids) in self.items_types().iter() {
    //         for compare_item_id in items_ids {
    //             if item_id == &compare_item_id {
    //                 return OptionalResult::Some(item_type);
    //             }
    //         }
    //     }
    //
    //     return OptionalResult::None;
    // }

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
