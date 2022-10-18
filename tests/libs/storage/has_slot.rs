use customize_nft::{libs::storage::StorageModule, structs::slot::Slot, Equip};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{managed_buffer, managed_token_id, rust_biguint};

use crate::testing_utils;

#[test]
fn should_return_true() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let token_id = managed_token_id!(b"HAT-1a1a11");
                let token_nonce = 1;
                let slot = ManagedBuffer::new_from_bytes(b"hat");

                let _ = sc.register_item(
                    Slot::new_from_buffer(slot),
                    managed_buffer!(b"Pirate Hat"),
                    token_id.clone(),
                    token_nonce,
                );

                assert_eq!(sc.has_slot(&token_id, token_nonce), true);
            },
        )
        .assert_ok();
}

#[test]
fn should_return_false() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            assert_eq!(sc.has_slot(&managed_token_id!(b"pirate hat"), 1), false);
        })
        .assert_ok();
}
