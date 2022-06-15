use customize_nft::{
    structs::{item::Item, item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes},
    Equip,
};
use elrond_wasm::{
    contract_base::ContractBase,
    types::{ManagedBuffer, MultiValueEncoded, SCResult, TokenIdentifier},
};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::testing_utils;

const PENGUIN_TOKEN_ID: &[u8] = testing_utils::PENGUIN_TOKEN_ID;

#[test]
fn customize_complete_flow() {
    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let item_to_desequip_slot = b"background";
    const ITEM_TO_DESEQUIP_ID: &[u8] = b"ITEM-a1a1a1";

    let item_to_equip_slot = b"hat";
    const ITEM_TO_EQUIP: &[u8] = b"HAT-b2b2b2";
    const ITEM_TO_EQUIP_NAME: &[u8] = b"new item";
    const NONCE: u64 = 30;

    DebugApi::dummy();
    setup.create_penguin_with_registered_item(
        NONCE,
        ITEM_TO_DESEQUIP_ID,
        NONCE,
        item_to_equip_slot,
        ItemAttributes::random(),
    );

    setup.register_item(
        item_to_desequip_slot,
        ITEM_TO_EQUIP,
        &ItemAttributes::random(),
    );

    setup.blockchain_wrapper.set_nft_balance_all_properties(
        &setup.first_user_address,
        ITEM_TO_EQUIP,
        NONCE,
        &rust_biguint!(1),
        &ItemAttributes::<DebugApi>::random(),
        0,
        Option::None,
        Option::Some(ITEM_TO_EQUIP_NAME),
        Option::None,
        &[],
    );

    let transfers =
        testing_utils::create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE), (ITEM_TO_EQUIP, NONCE)]);

    // 2. ACT
    let (sc_result, tx_result) = setup.customize(transfers, managed_buffer!(item_to_equip_slot));

    // 3. ASSERT
    tx_result.assert_ok();
    assert_eq!(sc_result, SCResult::Ok(1u64));

    // penguin sent burned
    setup.assert_is_burn(PENGUIN_TOKEN_ID, NONCE);

    // item equip on SC
    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.cf_wrapper.address_ref(),
            ITEM_TO_EQUIP,
            NONCE
        ),
        rust_biguint!(1)
    );

    // item desequipped received
    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            ITEM_TO_DESEQUIP_ID,
            NONCE
        ),
        rust_biguint!(1)
    );

    // new desquiped penguin received
    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            1u64
        ),
        rust_biguint!(1)
    );

    // do slot
    setup.blockchain_wrapper.check_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Option::Some(&PenguinAttributes::<DebugApi>::new(&[(
            &managed_buffer!(item_to_desequip_slot),
            Item {
                token: TokenIdentifier::from_esdt_bytes(ITEM_TO_EQUIP),
                nonce: NONCE,
                name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework force us to do this
            },
        )])),
    );
}

#[test]
fn customize_nothing_to_desequip_and_equip() {
    const NONCE: u64 = 30;

    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();
    setup.create_penguin_empty(NONCE);

    let transfers = testing_utils::create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE)]);

    // 2. ACT
    let tx_result = setup.blockchain_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        &transfers,
        |sc| {
            let managed_slots = MultiValueEncoded::<DebugApi, ManagedBuffer<DebugApi>>::new();

            let _ = sc.customize(sc.call_value().all_esdt_transfers(), managed_slots);
        },
    );

    // 3. ASSERT
    tx_result.assert_user_error(
        "You must either provide at least one penguin and one item OR provide a slot to desequip.",
    );
}
