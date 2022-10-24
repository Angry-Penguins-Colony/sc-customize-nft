use customize_nft::structs::slot::{Slot, ERR_MUST_BE_LOWERCASE};
use elrond_wasm::{elrond_codec::TopDecode, types::ManagedBuffer};
use elrond_wasm_debug::DebugApi;

use crate::testing_utils;

#[test]
fn works_if_lowercase() {
    DebugApi::dummy();

    let input_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(b"hat");

    let actual_output = Slot::<DebugApi>::top_decode(input_buffer).unwrap();

    assert_eq!(actual_output, Slot::<DebugApi>::new_from_bytes(b"hat"));
}

#[test]
fn panic_if_uppercase() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_| {
            let input_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(b"HAT");

            let actual_output = Slot::<DebugApi>::top_decode(input_buffer).unwrap();

            assert_eq!(actual_output, Slot::<DebugApi>::new_from_bytes(b"hat"));
        })
        .assert_user_error(std::str::from_utf8(ERR_MUST_BE_LOWERCASE).unwrap());
}
