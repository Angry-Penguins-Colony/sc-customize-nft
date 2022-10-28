use customize_nft::{
    constants::ERR_CANNOT_UNEQUIP_EMPTY_SLOT,
    libs::equippable_uris::EquippableUrisModule,
    structs::{
        equippable_attributes::EquippableAttributes,
        equippable_attributes_to_render::EquippableAttributesToRender,
        item::Item,
        slot::{Slot, ERR_MUST_BE_LOWERCASE},
    },
};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::testing_utils::{self, New, TestItemAttributes};

const EQUIPPABLE_TOKEN_ID: &[u8] = testing_utils::EQUIPPABLE_TOKEN_ID;

#[test]
fn customize_only_unequip() {
    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"background";
    const ITEM_TO_UNEQUIP_ID: &[u8] = b"BG-a1a1a1";
    const ITEM_TO_UNEQUIP_NAME: &[u8] = b"Some Item";
    const ITEM_TO_UNEQUIP_NONCE: u64 = 42;
    const EQUIPPABLE_NONCE: u64 = 30;

    DebugApi::dummy();

    setup.create_equippable_with_registered_item(
        EQUIPPABLE_NONCE,
        ITEM_TO_UNEQUIP_ID,
        ITEM_TO_UNEQUIP_NONCE,
        slot,
        TestItemAttributes {},
        ITEM_TO_UNEQUIP_NAME,
    );

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes_before_custom = EquippableAttributes::new(&[Item {
                    name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
                    slot: Slot::new_from_bytes(slot),
                }]);

                let mut attributes_after_custom = attributes_before_custom.clone();
                attributes_after_custom.empty_slot(&Slot::new_from_bytes(slot));

                sc.uris_of_attributes(&EquippableAttributesToRender {
                    attributes: attributes_before_custom,
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                })
                .set(ManagedBuffer::<DebugApi>::new_from_bytes(
                    b"https://ipfs.io/ipfs/this is a cid",
                ));

                sc.uris_of_attributes(&EquippableAttributesToRender {
                    attributes: attributes_after_custom,
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                })
                .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/empty"));
            },
        )
        .assert_ok();

    let transfers =
        testing_utils::create_esdt_transfers(&[(EQUIPPABLE_TOKEN_ID, EQUIPPABLE_NONCE)]);

    // 2. ACT
    let (sc_result, tx_result) = setup.customize(transfers, &[slot]);

    // 3. ASSERT
    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // equippable & items sent burned
    setup.assert_is_burn(EQUIPPABLE_TOKEN_ID, EQUIPPABLE_NONCE);

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            ITEM_TO_UNEQUIP_ID,
            ITEM_TO_UNEQUIP_NONCE
        ),
        rust_biguint!(1),
        "Item unequipped should be received"
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            EQUIPPABLE_TOKEN_ID,
            1u64
        ),
        rust_biguint!(1),
        "Equippable NFT should be received"
    );

    let mut attributes_after_custom = EquippableAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
        slot: Slot::new_from_bytes(slot),
    }]);
    attributes_after_custom.empty_slot(&Slot::new_from_bytes(slot));

    // is equippable empty
    setup.blockchain_wrapper.check_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Option::Some(&attributes_after_custom),
    );

    setup.assert_uris(EQUIPPABLE_TOKEN_ID, 1, &[b"https://ipfs.io/ipfs/empty"]);
}

#[test]
fn panic_if_uppercase_slot() {
    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const SLOT_LOWERCASE: &[u8] = b"background";
    const SLOT_UPPERCASE: &[u8] = b"BACKGROUND";
    const ITEM_TO_UNEQUIP_ID: &[u8] = b"BG-a1a1a1";
    const ITEM_TO_UNEQUIP_NAME: &[u8] = b"Some Item";
    const ITEM_TO_UNEQUIP_NONCE: u64 = 42;
    const EQUIPPABLE_NONCE: u64 = 30;

    DebugApi::dummy();

    setup.create_equippable_with_registered_item(
        EQUIPPABLE_NONCE,
        ITEM_TO_UNEQUIP_ID,
        ITEM_TO_UNEQUIP_NONCE,
        SLOT_LOWERCASE,
        TestItemAttributes {},
        ITEM_TO_UNEQUIP_NAME,
    );

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes_before_custom = EquippableAttributes::new(&[Item {
                    name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
                    slot: Slot::new_from_bytes(SLOT_LOWERCASE),
                }]);

                let mut attributes_after_custom = attributes_before_custom.clone();
                attributes_after_custom.empty_slot(&Slot::new_from_bytes(SLOT_LOWERCASE));

                sc.uris_of_attributes(&EquippableAttributesToRender {
                    attributes: attributes_before_custom,
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                })
                .set(ManagedBuffer::<DebugApi>::new_from_bytes(
                    b"https://ipfs.io/ipfs/this is a cid",
                ));

                sc.uris_of_attributes(&EquippableAttributesToRender {
                    attributes: attributes_after_custom,
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                })
                .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/empty"));
            },
        )
        .assert_ok();

    let transfers =
        testing_utils::create_esdt_transfers(&[(EQUIPPABLE_TOKEN_ID, EQUIPPABLE_NONCE)]);

    // 2. ACT
    let (_, tx_result) = setup.customize(transfers, &[SLOT_UPPERCASE]);

    // 3. ASSERT
    tx_result.assert_user_error(std::str::from_utf8(ERR_MUST_BE_LOWERCASE).unwrap())
}

#[test]
fn panic_when_unequip_twice_the_same_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"background";
    const ITEM_TO_UNEQUIP_ID: &[u8] = b"BG-a1a1a1";
    const ITEM_TO_UNEQUIP_NAME: &[u8] = b"Some Item";
    const ITEM_TO_UNEQUIP_NONCE: u64 = 42;
    const EQUIPPABLE_NONCE: u64 = 30;

    DebugApi::dummy();

    setup.create_equippable_with_registered_item(
        EQUIPPABLE_NONCE,
        ITEM_TO_UNEQUIP_ID,
        ITEM_TO_UNEQUIP_NONCE,
        slot,
        TestItemAttributes {},
        ITEM_TO_UNEQUIP_NAME,
    );

    // setup CID
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes_before_custom = EquippableAttributesToRender {
                    attributes: EquippableAttributes::new(&[Item {
                        name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
                        slot: Slot::new_from_bytes(slot),
                    }]),
                    name: ManagedBuffer::new_from_bytes(EQUIPPABLE_TOKEN_ID),
                };
                sc.uris_of_attributes(&attributes_before_custom).set(
                    ManagedBuffer::<DebugApi>::new_from_bytes(b"https://ipfs.io/ipfs/before"),
                );

                let attributes_after_custom = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::empty(),
                    name: ManagedBuffer::new_from_bytes(EQUIPPABLE_TOKEN_ID),
                };
                sc.uris_of_attributes(&attributes_after_custom).set(
                    ManagedBuffer::<DebugApi>::new_from_bytes(b"https://ipfs.io/ipfs/after"),
                );
            },
        )
        .assert_ok();

    let transfers =
        testing_utils::create_esdt_transfers(&[(EQUIPPABLE_TOKEN_ID, EQUIPPABLE_NONCE)]);

    // 2. ACT
    let (_, tx_result) = setup.customize(transfers.clone(), &[slot, slot]);

    // 3. ASSERT
    tx_result.assert_user_error(ERR_CANNOT_UNEQUIP_EMPTY_SLOT);
}

#[test]
fn panic_when_unequip_on_empty_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"background";
    const NONCE: u64 = 30;

    setup.create_empty_equippable(NONCE);

    // setup CID
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::empty(),
                    name: ManagedBuffer::new_from_bytes(EQUIPPABLE_TOKEN_ID),
                };
                sc.uris_of_attributes(&attributes)
                    .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/empty"));
            },
        )
        .assert_ok();

    let transfers = testing_utils::create_esdt_transfers(&[(EQUIPPABLE_TOKEN_ID, NONCE)]);

    // 2. ACT
    let (_, tx_result) = setup.customize(transfers.clone(), &[slot]);

    // 3. ASSERT
    tx_result.assert_user_error(ERR_CANNOT_UNEQUIP_EMPTY_SLOT);
}
