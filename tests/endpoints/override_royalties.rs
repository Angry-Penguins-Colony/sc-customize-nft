use customize_nft::{libs::storage::StorageModule, EndpointWrappers, Equip, ERR_BAD_ROYALTIES};
use elrond_wasm::types::BigUint;
use elrond_wasm_debug::rust_biguint;

use crate::testing_utils;

#[test]
fn works() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.override_royalties(BigUint::from(1u64));

                assert_eq!(sc.royalties_overrided().get(), BigUint::from(1u64));
            },
        )
        .assert_ok();
}

#[test]
fn panic_if_not_owner() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.first_user_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| sc.call_override_royalties(),
        )
        .assert_user_error("Endpoint can only be called by owner");
}

#[test]
fn works_if_equals_zero() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| sc.override_royalties(BigUint::from(0u64)),
        )
        .assert_ok();
}

#[test]
fn works_if_equals_10000() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| sc.override_royalties(BigUint::from(10000u64)),
        )
        .assert_ok();
}

#[test]
fn panic_if_greater_than_10000() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| sc.override_royalties(BigUint::from(10001u64)),
        )
        .assert_user_error(ERR_BAD_ROYALTIES);
}
