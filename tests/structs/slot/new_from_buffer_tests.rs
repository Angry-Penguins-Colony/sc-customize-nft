use customize_nft::structs::slot::{Slot, ERR_MUST_BE_LOWERCASE, ERR_UNSUPPORTED_CHARACTERS};
use elrond_wasm_debug::DebugApi;

use crate::testing_utils;
use std::str;

#[test]
fn new_should_not_panic() {
    DebugApi::dummy();

    Slot::<DebugApi>::new_from_bytes(b"hat");
}

#[test]
fn panic_if_has_colon() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_sc| {
            Slot::<DebugApi>::new_from_bytes(b"ha:t");
        })
        .assert_user_error(str::from_utf8(ERR_UNSUPPORTED_CHARACTERS).unwrap());
}

#[test]
fn panic_if_has_semicolon() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_sc| {
            Slot::<DebugApi>::new_from_bytes(b"ha;t");
        })
        .assert_user_error(str::from_utf8(ERR_UNSUPPORTED_CHARACTERS).unwrap());
}

#[test]
fn panic_if_uppercase() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_sc| {
            Slot::<DebugApi>::new_from_bytes(b"HAT");
        })
        .assert_user_error(str::from_utf8(ERR_MUST_BE_LOWERCASE).unwrap());
}
