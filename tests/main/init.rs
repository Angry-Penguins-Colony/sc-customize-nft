use customize_nft::{
    constants::ERR_INIT_MISSING_NUMBER_FORMAT, libs::storage::StorageModule, Equip,
};
use elrond_wasm_debug::{
    managed_buffer, managed_token_id, rust_biguint, testing_framework::BlockchainStateWrapper,
};

use crate::testing_utils;

#[test]
fn should_add_backslash_to_gateway() {
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        customize_nft::contract_obj,
        testing_utils::WASM_PATH,
    );

    // deploy contract
    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init(
                managed_token_id!(b"PEN-a1a1a1"),
                managed_buffer!(b"https://ipfs.io/ipfs"),
                managed_buffer!(b"Equippable #{number}"),
            );
        })
        .assert_ok();

    blockchain_wrapper
        .execute_query(&cf_wrapper, |sc| {
            assert_eq!(
                sc.ipfs_gateway().get(),
                managed_buffer!(b"https://ipfs.io/ipfs/"),
                "The gateway should have been append with a backslash"
            );
        })
        .assert_ok();
}

#[test]
fn should_throw_error_if_missing_format() {
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        customize_nft::contract_obj,
        testing_utils::WASM_PATH,
    );

    // deploy contract
    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init(
                managed_token_id!(b"PEN-a1a1a1"),
                managed_buffer!(b"https://ipfs.io/ipfs"),
                managed_buffer!(b"Equippable #"),
            );
        })
        .assert_user_error(ERR_INIT_MISSING_NUMBER_FORMAT);
}
