use customize_nft::EndpointWrappers;
use elrond_wasm_debug::rust_biguint;

use crate::testing_utils::{self, TestItemAttributes};

#[test]
fn ok_if_owner() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const TOKEN: &[u8] = b"HAT-a1a1a1";
    const NONCE: u64 = 1;

    setup.register_and_fill_item(b"hat", b"pirate hat", TOKEN, NONCE, &TestItemAttributes {});
    assert_eq!(
        setup
            .blockchain_wrapper
            .get_esdt_balance(&setup.cf_wrapper.address_ref(), TOKEN, NONCE),
        rust_biguint!(2),
        "The sc should own the token"
    );

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.call_claims_items();
            },
        )
        .assert_ok();

    assert_eq!(
        setup
            .blockchain_wrapper
            .get_esdt_balance(&setup.owner_address, TOKEN, NONCE),
        rust_biguint!(2),
        "The owner should own the token"
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
                sc.call_claims_items();
            },
        )
        .assert_user_error("Endpoint can only be called by owner");
}
