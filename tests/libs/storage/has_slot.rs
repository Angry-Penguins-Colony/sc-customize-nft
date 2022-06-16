use customize_nft::{libs::storage::StorageModule, Equip};
use elrond_wasm::types::{ManagedBuffer, MultiValueEncoded, TokenIdentifier};
use elrond_wasm_debug::{managed_token_id, rust_biguint, DebugApi};

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
                let token_id = managed_token_id!(b"pirate hat");
                let slot = ManagedBuffer::new_from_bytes(b"hat");

                let mut managed_items_ids =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                managed_items_ids.push(token_id.clone());

                let _ = sc.register_item(slot, managed_items_ids);

                assert_eq!(sc.has_slot(&token_id), true);
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
            let token_id = managed_token_id!(b"pirate hat");

            assert_eq!(sc.has_slot(&token_id), false);
        })
        .assert_ok();
}
