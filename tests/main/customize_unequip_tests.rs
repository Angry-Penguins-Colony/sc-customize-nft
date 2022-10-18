use customize_nft::{
    constants::ERR_CANNOT_UNEQUIP_EMPTY_SLOT,
    libs::storage::StorageModule,
    structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item},
};
use elrond_wasm::elrond_codec::multi_types::MultiValue2;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm_debug::{rust_biguint, DebugApi};

use crate::{
    args_set_cid_of,
    testing_utils::{self, TestItemAttributes},
};

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
    );

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.ipfs_gateway()
                    .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/"));

                let attributes_before_custom = EquippableNftAttributes::new(&[Item {
                    name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
                    slot: ManagedBuffer::new_from_bytes(slot),
                }]);

                let mut attributes_after_custom = attributes_before_custom.clone();
                attributes_after_custom.empty_slot(&ManagedBuffer::new_from_bytes(slot));

                sc.set_cid_of(args_set_cid_of!(
                    attributes_before_custom,
                    ManagedBuffer::<DebugApi>::new_from_bytes(b"this is a cid")
                ));

                sc.set_cid_of(args_set_cid_of!(
                    attributes_after_custom,
                    ManagedBuffer::new_from_bytes(b"empty")
                ));
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

    let mut attributes_after_custom = EquippableNftAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
        slot: ManagedBuffer::new_from_bytes(slot),
    }]);
    attributes_after_custom.empty_slot(&ManagedBuffer::new_from_bytes(slot));

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
fn unequip_should_ignore_case_of_slot() {
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
    );

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.ipfs_gateway()
                    .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/"));

                let attributes_before_custom = EquippableNftAttributes::new(&[Item {
                    name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
                    slot: ManagedBuffer::new_from_bytes(SLOT_LOWERCASE),
                }]);

                let mut attributes_after_custom = attributes_before_custom.clone();
                attributes_after_custom.empty_slot(&ManagedBuffer::new_from_bytes(SLOT_LOWERCASE));

                sc.set_cid_of(args_set_cid_of!(
                    attributes_before_custom,
                    ManagedBuffer::<DebugApi>::new_from_bytes(b"this is a cid")
                ));

                sc.set_cid_of(args_set_cid_of!(
                    attributes_after_custom,
                    ManagedBuffer::new_from_bytes(b"empty")
                ));
            },
        )
        .assert_ok();

    let transfers =
        testing_utils::create_esdt_transfers(&[(EQUIPPABLE_TOKEN_ID, EQUIPPABLE_NONCE)]);

    // 2. ACT
    let (sc_result, tx_result) = setup.customize(transfers, &[SLOT_UPPERCASE]);

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

    // is equippable empty
    let mut attributes_after_custom = EquippableNftAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
        slot: ManagedBuffer::new_from_bytes(SLOT_LOWERCASE),
    }]);
    attributes_after_custom.empty_slot(&ManagedBuffer::new_from_bytes(SLOT_LOWERCASE));

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
    );

    // setup CID
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes_before_custom = EquippableNftAttributes::new(&[Item {
                    name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
                    slot: ManagedBuffer::new_from_bytes(slot),
                }]);

                sc.set_cid_of(args_set_cid_of!(
                    attributes_before_custom,
                    ManagedBuffer::<DebugApi>::new_from_bytes(b"this is a cid")
                ));

                sc.set_cid_of(args_set_cid_of!(
                    EquippableNftAttributes::<DebugApi>::empty(),
                    ManagedBuffer::new_from_bytes(b"empty")
                ));
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

    let slot = b"Background";
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
                sc.set_cid_of(args_set_cid_of!(
                    EquippableNftAttributes::<DebugApi>::empty(),
                    ManagedBuffer::new_from_bytes(b"empty")
                ));
            },
        )
        .assert_ok();

    let transfers = testing_utils::create_esdt_transfers(&[(EQUIPPABLE_TOKEN_ID, NONCE)]);

    // 2. ACT
    let (_, tx_result) = setup.customize(transfers.clone(), &[slot]);

    // 3. ASSERT
    tx_result.assert_user_error(ERR_CANNOT_UNEQUIP_EMPTY_SLOT);
}
