use customize_nft::{EndpointWrappers, Equip};
use elrond_wasm_debug::rust_biguint;

use crate::testing_utils;

#[test]
fn works() {
    const EGLD_AMOUNT: u64 = 1_000_000;
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.cf_wrapper.address_ref(), &rust_biguint!(EGLD_AMOUNT));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim();
            },
        )
        .assert_ok();

    assert_eq!(
        setup
            .blockchain_wrapper
            .get_egld_balance(&setup.owner_address),
        rust_biguint!(EGLD_AMOUNT),
        "The owner should have received the EGLD from endpoint claim."
    );
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
            |sc| {
                sc.call_claim();
            },
        )
        .assert_user_error("Endpoint can only be called by owner");
}
