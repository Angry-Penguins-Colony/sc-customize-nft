use elrond_wasm::types::SCResult;
use elrond_wasm_debug::rust_biguint;
use equip_penguin::{item_attributes::ItemAttributes, item_slot::ItemSlot};
use utils::{create_esdt_transfers, execute_for_all_slot};

mod utils;

const PENGUIN_TOKEN_ID: &[u8] = utils::PENGUIN_TOKEN_ID;

#[test]
fn test_desequip() {
    execute_for_all_slot(|slot| {
        const ITEM_TO_DESEQUIP_ID: &[u8] = b"ITEM-a";
        const NONCE: u64 = 30;

        // 1. ARRANGE
        let mut setup = utils::setup(equip_penguin::contract_obj);

        setup.create_penguin_with_registered_item(
            NONCE,
            ITEM_TO_DESEQUIP_ID,
            NONCE,
            slot.clone(),
            ItemAttributes::random(),
        );

        let transfers = create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE)]);

        // 2. ACT
        let (sc_result, tx_result) = setup.desequip(slot.clone(), transfers, NONCE);

        // 3. ASSERT
        tx_result.assert_ok();
        assert_eq!(sc_result, SCResult::Ok(1u64));

        // item desequipped received
        assert_eq!(
            setup.blockchain_wrapper.get_esdt_balance(
                &setup.first_user_address,
                ITEM_TO_DESEQUIP_ID,
                NONCE
            ),
            rust_biguint!(1)
        );

        // penguin sent burned
        setup.assert_is_burn(PENGUIN_TOKEN_ID, NONCE);

        // new desquiped penguin received
        assert_eq!(
            setup.blockchain_wrapper.get_esdt_balance(
                &setup.first_user_address,
                PENGUIN_TOKEN_ID,
                1u64
            ),
            rust_biguint!(1)
        );
    });
}

#[test]
fn test_desequip_with_slot_none() {
    const SLOT: ItemSlot = ItemSlot::None;
    const ITEM_TO_DESEQUIP_ID: &[u8] = b"ITEM-a";
    const NONCE: u64 = 30;

    // 1. ARRANGE
    let mut setup = utils::setup(equip_penguin::contract_obj);

    setup.create_penguin_with_registered_item(
        NONCE,
        ITEM_TO_DESEQUIP_ID,
        NONCE,
        ItemSlot::Hat.clone(), /* we don't use const SLOT, because ItemSlot::None make panics */
        ItemAttributes::random(),
    );

    let transfers = create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE)]);

    // 2. ACT
    let (_, tx_result) = setup.desequip(SLOT.clone(), transfers, NONCE);

    // 3. ASSERT
    tx_result.assert_user_error("Slot value must be different to ItemSlot::None.");
}
