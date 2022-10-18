use customize_nft::{constants::ERR_GATEWAY_NEEDS_SLASH, Equip};
use elrond_wasm_debug::{
    managed_buffer, managed_token_id, rust_biguint, testing_framework::BlockchainStateWrapper,
};

use crate::testing_utils;

#[test]
fn work_at_init() {
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
                managed_buffer!(b"https://ipfs.io/ipfs/"),
            );
        })
        .assert_ok();
}

#[test]
fn should_panic_if_no_slash_to_gateway() {
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
            );
        })
        .assert_user_error(ERR_GATEWAY_NEEDS_SLASH);
}
