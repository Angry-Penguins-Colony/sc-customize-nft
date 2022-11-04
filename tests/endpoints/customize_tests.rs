use customize_nft::{
    constants::ERR_NEED_ONE_ITEM_OR_UNEQUIP_SLOT,
    libs::{customize::CustomizeModule, equippable_uris::EquippableUrisModule},
    structs::{equippable_attributes::EquippableAttributes, item::Item},
};
use elrond_wasm::types::{ManagedBuffer, MultiValueEncoded};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::testing_utils::{self, New, TestItemAttributes};

const EQUIPPABLE_TOKEN_ID: &[u8] = testing_utils::EQUIPPABLE_TOKEN_ID;

#[test]
fn customize_complete_flow() {
    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const EQUIPPABLE_TOKEN_NONCE: u64 = 5;

    const ITEM_TO_UNEQUIP_SLOT: &[u8] = b"background";
    const ITEM_TO_UNEQUIP_ID: &[u8] = b"ITEM-a1a1a1";
    const ITEM_TO_UNEQUIP_NONCE: u64 = 30;
    const ITEM_TO_UNEQUIP_NAME: &[u8] = b"Turquoise Background";

    const ITEM_TO_EQUIP_SLOT: &[u8] = b"hat";
    const ITEM_TO_EQUIP_ID: &[u8] = b"HAT-b2b2b2";
    const ITEM_TO_EQUIP_NONCE: u64 = 42;
    const ITEM_TO_EQUIP_NAME: &[u8] = b"Pirate Hat";

    DebugApi::dummy();

    // Create an equippable NFT with item to unequip
    setup.create_equippable_with_registered_item(
        EQUIPPABLE_TOKEN_NONCE,
        ITEM_TO_UNEQUIP_ID,
        ITEM_TO_UNEQUIP_NONCE,
        ITEM_TO_UNEQUIP_SLOT,
        TestItemAttributes {},
        ITEM_TO_UNEQUIP_NAME,
    );

    // Register item to equip
    setup.register_and_fill_item(
        ITEM_TO_EQUIP_SLOT,
        ITEM_TO_EQUIP_NAME,
        ITEM_TO_EQUIP_ID,
        ITEM_TO_EQUIP_NONCE,
        &TestItemAttributes {},
    );

    // Add to user an item to equip
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        ITEM_TO_EQUIP_ID,
        ITEM_TO_EQUIP_NONCE,
        &rust_biguint!(1),
        &TestItemAttributes {},
    );

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes_before_custom = EquippableAttributes::new(&[Item {
                    name: managed_buffer!(ITEM_TO_UNEQUIP_NAME),
                    slot: managed_buffer!(ITEM_TO_UNEQUIP_SLOT),
                }]);

                let mut attributes_after_custom = attributes_before_custom.clone();

                attributes_after_custom.empty_slot(&managed_buffer!(ITEM_TO_UNEQUIP_SLOT));
                attributes_after_custom.set_item_if_empty(
                    &managed_buffer!(ITEM_TO_EQUIP_SLOT),
                    Some(managed_buffer!(ITEM_TO_EQUIP_NAME)),
                );

                sc.uris_of_attributes(
                    &attributes_before_custom,
                    &managed_buffer!(EQUIPPABLE_TOKEN_ID),
                )
                .set(managed_buffer!(b"https://ipfs.io/ipfs/cid before custom"));
                sc.uris_of_attributes(
                    &attributes_after_custom,
                    &managed_buffer!(EQUIPPABLE_TOKEN_ID),
                )
                .set(managed_buffer!(b"https://ipfs.io/ipfs/cid after custom"));
            },
        )
        .assert_ok();

    let transfers = testing_utils::create_esdt_transfers(&[
        (EQUIPPABLE_TOKEN_ID, EQUIPPABLE_TOKEN_NONCE),
        (ITEM_TO_EQUIP_ID, ITEM_TO_EQUIP_NONCE),
    ]);

    // 2. ACT
    let (sc_result, tx_result) = setup.customize(transfers, &[ITEM_TO_UNEQUIP_SLOT]);

    // 3. ASSERT
    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // Equippable NFT sent burned
    setup.assert_is_burn(EQUIPPABLE_TOKEN_ID, ITEM_TO_EQUIP_NONCE);

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.cf_wrapper.address_ref(),
            ITEM_TO_EQUIP_ID,
            ITEM_TO_EQUIP_NONCE
        ),
        rust_biguint!(3),
        "The user should have send the item to equip on the smart contract + the 2 items from register_item() function."
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            ITEM_TO_UNEQUIP_ID,
            ITEM_TO_UNEQUIP_NONCE
        ),
        rust_biguint!(1),
        "The user should have received the item unequipped"
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            EQUIPPABLE_TOKEN_ID,
            1u64
        ),
        rust_biguint!(1),
        "The user should have received the penguin"
    );

    let mut attributes_after_custom = EquippableAttributes::<DebugApi>::new(&[Item {
        name: managed_buffer!(ITEM_TO_UNEQUIP_NAME),
        slot: managed_buffer!(ITEM_TO_UNEQUIP_SLOT),
    }]);
    attributes_after_custom.empty_slot(&managed_buffer!(ITEM_TO_UNEQUIP_SLOT));
    attributes_after_custom.set_item_if_empty(
        &managed_buffer!(ITEM_TO_EQUIP_SLOT),
        Some(managed_buffer!(ITEM_TO_EQUIP_NAME)),
    );

    setup.blockchain_wrapper.check_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Option::Some(&attributes_after_custom),
    );

    setup.assert_uris(
        EQUIPPABLE_TOKEN_ID,
        1,
        &[b"https://ipfs.io/ipfs/cid after custom"],
    );
}

#[test]
fn customize_nothing_to_unequip_and_equip() {
    const NONCE: u64 = 30;

    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();
    setup.create_empty_equippable(NONCE);

    let transfers = testing_utils::create_esdt_transfers(&[(EQUIPPABLE_TOKEN_ID, NONCE)]);

    // 2. ACT
    let tx_result = setup.blockchain_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        &transfers,
        |sc| {
            let managed_slots = MultiValueEncoded::<DebugApi, ManagedBuffer<DebugApi>>::new();

            let _ = sc.customize(managed_slots);
        },
    );

    // 3. ASSERT
    tx_result.assert_user_error(ERR_NEED_ONE_ITEM_OR_UNEQUIP_SLOT);
}
