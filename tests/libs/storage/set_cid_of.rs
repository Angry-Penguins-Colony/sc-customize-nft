use customize_nft::{libs::storage::StorageModule, structs::penguin_attributes::PenguinAttributes};
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::testing_utils;

#[test]
fn should_set_if_empty() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let cid_bytes = b"some cid";

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let attributes = PenguinAttributes::<DebugApi>::empty();
            sc.set_cid_of(&attributes, managed_buffer!(cid_bytes));

            assert_eq!(sc.cid_of(&attributes).get(), managed_buffer!(cid_bytes));
        })
        .assert_ok();
}

#[test]
fn should_set_if_not_emtpy() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let first_cid_bytes = b"some cid";
    let second_cid_bytes = b"another cid";

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let attributes = PenguinAttributes::<DebugApi>::empty();

            sc.set_cid_of(&attributes, managed_buffer!(first_cid_bytes));
            assert_eq!(
                sc.cid_of(&attributes).get(),
                managed_buffer!(first_cid_bytes)
            );

            sc.set_cid_of(&attributes, managed_buffer!(second_cid_bytes));
            assert_eq!(
                sc.cid_of(&attributes).get(),
                managed_buffer!(second_cid_bytes),
                "first_cid_bytes should be overwrited by second_cid_bytes"
            );
        })
        .assert_ok();
}
