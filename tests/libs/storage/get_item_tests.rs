use customize_nft::structs::item_attributes::ItemAttributes;
use customize_nft::*;
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::DebugApi;

use crate::testing_utils;

const HAT_TOKEN_ID: &[u8] = testing_utils::HAT_TOKEN_ID;

#[test]
fn get_item() {
    DebugApi::dummy();

    let slot = &ManagedBuffer::new_from_bytes(b"hat");
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.register_item(slot.clone(), HAT_TOKEN_ID, &ItemAttributes::random());

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);

            let opt_actual_slot = sc.get_item_slot(&hat_token).into_option();

            assert_eq!(opt_actual_slot.is_some(), true);
            assert_eq!(&opt_actual_slot.unwrap(), slot);
        })
        .assert_ok();
}

#[test]
fn return_none_if_no_token_id() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let not_existing_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(b"NOT_TOKEN_ID");

            let opt_slot = sc.get_item_slot(&not_existing_token).into_option();

            assert_eq!(opt_slot.is_none(), true);
        })
        .assert_ok();
}
