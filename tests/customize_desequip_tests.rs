use elrond_wasm::types::SCResult;
use elrond_wasm_debug::{rust_biguint, DebugApi};
use customize_nft::structs::{
    item_attributes::ItemAttributes, item_slot::ItemSlot, penguin_attributes::PenguinAttributes,
};

mod testing_utils;

const PENGUIN_TOKEN_ID: &[u8] = testing_utils::PENGUIN_TOKEN_ID;

#[test]
fn customize_only_desequip() {
    testing_utils::execute_for_all_slot(|slot| {
        const ITEM_TO_DESEQUIP_ID: &[u8] = b"ITEM-a1a1a1";
        const NONCE: u64 = 30;

        // 1. ARRANGE
        let mut setup = testing_utils::setup(customize_nft::contract_obj);

        setup.create_penguin_with_registered_item(
            NONCE,
            ITEM_TO_DESEQUIP_ID,
            NONCE,
            slot.clone(),
            ItemAttributes::random(),
        );

        let transfers = testing_utils::create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE)]);

        // 2. ACT
        let (sc_result, tx_result) = setup.customize(transfers, slot.clone());

        // 3. ASSERT
        tx_result.assert_ok();
        assert_eq!(sc_result, SCResult::Ok(1u64));

        // penguin&items sent burned
        setup.assert_is_burn(PENGUIN_TOKEN_ID, NONCE);

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

        // is pinguin empty
        setup.blockchain_wrapper.check_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            1,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::empty(),
        );
    });
}

#[test]
fn test_desequip_with_slot_none() {
    const SLOT: ItemSlot = ItemSlot::None;
    const ITEM_TO_DESEQUIP_ID: &[u8] = b"ITEM-a";
    const NONCE: u64 = 30;

    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.create_penguin_with_registered_item(
        NONCE,
        ITEM_TO_DESEQUIP_ID,
        NONCE,
        ItemSlot::Hat.clone(), /* we don't use const SLOT, because ItemSlot::None make panics */
        ItemAttributes::random(),
    );

    let transfers = testing_utils::create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE)]);

    // 2. ACT
    let (_, tx_result) = setup.customize(transfers, SLOT.clone());

    // 3. ASSERT
    tx_result.assert_user_error("Slot value must be different to ItemSlot::None.");
}
