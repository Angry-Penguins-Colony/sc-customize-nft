use elrond_wasm::types::ManagedVec;
use elrond_wasm_debug::{managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::item_slot::ItemSlot;
use equip_penguin::*;

mod utils;

const HAT_TOKEN_ID: &[u8] = utils::HAT_TOKEN_ID;
const ANOTHER_HAT_TOKEN_ID: &[u8] = utils::HAT_2_TOKEN_ID;

#[test]
fn test_register_item() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    utils::register_item(&mut setup, ItemSlot::Hat, HAT_TOKEN_ID);
    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let managed_token_id = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);
            let mut managed_items_ids = ManagedVec::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id.clone());

            let result = sc.items_slot(&managed_token_id!(HAT_TOKEN_ID)).get();

            assert_eq!(result, ItemSlot::Hat);

            StateChange::Commit
        },
    );
}

#[test]
fn register_item_should_insert() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    utils::register_item(&mut setup, ItemSlot::Hat, HAT_TOKEN_ID);
    utils::register_item(&mut setup, ItemSlot::Hat, ANOTHER_HAT_TOKEN_ID);
    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_query(&setup.cf_wrapper, |sc| {
        let result = sc.items_slot(&managed_token_id!(HAT_TOKEN_ID)).get();

        assert_eq!(result, ItemSlot::Hat);

        let result2 = sc
            .items_slot(&managed_token_id!(ANOTHER_HAT_TOKEN_ID))
            .get();

        assert_eq!(result2, ItemSlot::Hat);
    });
}
