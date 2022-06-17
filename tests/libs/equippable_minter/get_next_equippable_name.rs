use customize_nft::{libs::equippable_minter::MintEquippableModule, Equip};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{
    managed_buffer, managed_token_id, rust_biguint, testing_framework::BlockchainStateWrapper,
    DebugApi,
};

use crate::testing_utils;

#[test]
fn should_work() {
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
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            let name = sc.get_next_equippable_name();

            assert_eq!(
                name,
                ManagedBuffer::<DebugApi>::new_from_bytes(b"Equippable #1")
            )
        })
        .assert_ok();
}
