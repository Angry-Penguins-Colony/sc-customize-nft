use elrond_wasm::types::TokenIdentifier;
use elrond_wasm_debug::DebugApi;
use elrond_wasm_debug::{managed_buffer, rust_biguint};
use equip_penguin::Equip;

mod testing_utils;

#[test]
fn get_token_name_test() {
    DebugApi::dummy();

    const ID: &[u8] = b"ITEM-a1a1a1";
    const NONCE: u64 = 1u64;
    const NAME: &[u8] = b"name";

    let mut setup = testing_utils::setup(equip_penguin::contract_obj);

    setup.blockchain_wrapper.set_nft_balance_all_properties(
        &setup.cf_wrapper.address_ref(),
        ID,
        NONCE,
        &rust_biguint!(1),
        &{},
        0,
        Option::None,
        Option::Some(NAME),
        Option::None,
        &[],
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let result = sc.get_token_name(&TokenIdentifier::from_esdt_bytes(ID), NONCE);
            assert_eq!(result.unwrap(), managed_buffer!(NAME))
        })
        .assert_ok();
}
