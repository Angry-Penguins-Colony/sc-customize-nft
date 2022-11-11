use customize_nft::Equip;
use elrond_wasm_debug::{managed_buffer, managed_token_id, rust_biguint};

use crate::testing_utils::{self, TestItemAttributes};

#[test]
fn ok_if_owner() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const TOKEN: &[u8] = b"HAT-a1a1a1";
    const NONCE: u64 = 1;

    let slot = b"hat";
    let item_name = b"pirate hat";
    setup.register_and_fill_item(slot, item_name, TOKEN, NONCE, &TestItemAttributes {});
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
                let output = sc.get_items();

                let mut iter = output.into_iter();

                assert_eq!(
                    iter.next().unwrap().into_tuple(),
                    (
                        managed_buffer!(slot),
                        managed_buffer!(item_name),
                        managed_token_id!(TOKEN),
                        NONCE
                    )
                );
                assert_eq!(iter.next().is_none(), true);
            },
        )
        .assert_ok();
}
