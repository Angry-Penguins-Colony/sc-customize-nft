use customize_nft::{libs::storage::StorageModule, structs::slot::Slot};
use elrond_wasm::types::TokenIdentifier;
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::testing_utils::{self, TestItemAttributes};

const HAT_TOKEN_ID: &[u8] = testing_utils::HAT_TOKEN_ID;

#[test]
fn should_returns_some() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const SLOT: &[u8] = b"hat";
    const ITEM_NONCE: u64 = 1u64;

    DebugApi::dummy();
    setup.register_and_fill_item(SLOT, HAT_TOKEN_ID, ITEM_NONCE, &TestItemAttributes {});

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);

            let actual_slot = sc.get_slot_of(&hat_token);
            assert_eq!(actual_slot, Slot::new_from_buffer(managed_buffer!(SLOT)));
        })
        .assert_ok();
}

#[test]
fn should_returns_none() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let not_existing_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(b"NOT_TOKEN_ID");

            let _ = sc.get_slot_of(&not_existing_token);
        })
        .assert_user_error("No slot found for NOT_TOKEN_ID.");
}
