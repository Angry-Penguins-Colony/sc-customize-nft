use crate::testing_utils::{New, TestItemAttributes};
use customize_nft::constants::{
    ERR_FIRST_PAYMENT_IS_EQUIPPABLE, ERR_MORE_THAN_ONE_ITEM_RECEIVED,
    ERR_NEED_ONE_ITEM_OR_UNEQUIP_SLOT,
};
use customize_nft::libs::equippable_uris::EquippableUrisModule;
use customize_nft::structs::equippable_attributes::EquippableAttributes;
use customize_nft::structs::equippable_attributes_to_render::EquippableAttributesToRender;
use customize_nft::structs::item::Item;
use customize_nft::structs::slot::Slot;
use elrond_wasm::types::{EsdtTokenType, ManagedBuffer};
use elrond_wasm_debug::managed_buffer;
use elrond_wasm_debug::tx_mock::TxInputESDT;
use elrond_wasm_debug::{rust_biguint, DebugApi};

use crate::testing_utils;

const EQUIPPABLE_TOKEN_ID: &[u8] = testing_utils::EQUIPPABLE_TOKEN_ID;
const HAT_TOKEN_ID: &[u8] = testing_utils::HAT_TOKEN_ID;
const NOT_EQUIPPABLE_TOKEN_ID: &[u8] = b"QUACK-a456e";

// create NFT on blockchain wrapper
#[test]
fn test_equip() {
    const EQUIPPABLE_TOKEN_NONCE: u64 = 65535;
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();

    let slot = b"hat";
    const ITEM_TO_EQUIP_ID: &[u8] = b"ITEM-a1a1a1";
    const ITEM_TO_EQUIP_NAME: &[u8] = b"item name";
    const ITEM_TO_EQUIP_NONCE: u64 = 600u64;

    setup.register_and_fill_items_all_properties(
        slot,
        ITEM_TO_EQUIP_NAME,
        ITEM_TO_EQUIP_ID,
        ITEM_TO_EQUIP_NONCE,
        &TestItemAttributes {},
        0,
        Option::None,
        Option::None,
        &[],
    );

    // add empty equippable to the USER
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        EQUIPPABLE_TOKEN_NONCE,
        &rust_biguint!(1),
        &EquippableAttributes::<DebugApi>::empty(),
    );

    // add item_to_equip_id
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        ITEM_TO_EQUIP_ID,
        ITEM_TO_EQUIP_NONCE,
        &rust_biguint!(1),
        &TestItemAttributes {},
    );

    // set cid
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes_before_custom = &EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::empty(),
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                };

                sc.uris_of_attributes(&attributes_before_custom).set(
                    ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/cid before custom"),
                );

                let attributes_after_custom = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::new(&[Item {
                        name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_NAME),
                        slot: Slot::new_from_buffer(managed_buffer!(slot)),
                    }]),
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                };

                sc.uris_of_attributes(&attributes_after_custom)
                    .set(ManagedBuffer::new_from_bytes(
                        b"https://ipfs.io/ipfs/after custom",
                    ));
            },
        )
        .assert_ok();

    let transfers = testing_utils::create_esdt_transfers(&[
        (EQUIPPABLE_TOKEN_ID, EQUIPPABLE_TOKEN_NONCE),
        (ITEM_TO_EQUIP_ID, ITEM_TO_EQUIP_NONCE),
    ]);

    let (sc_result, tx_result) = setup.equip(transfers);

    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // the transfered equippable is burn
    setup.assert_is_burn(&EQUIPPABLE_TOKEN_ID, EQUIPPABLE_TOKEN_NONCE);
    setup.assert_is_burn(&HAT_TOKEN_ID, ITEM_TO_EQUIP_NONCE);

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            EQUIPPABLE_TOKEN_ID,
            1
        ),
        rust_biguint!(1),
        "The new equippable NFT has been received."
    );

    // the NEW equippable has the right attributes
    setup.blockchain_wrapper.check_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        1u64,
        &rust_biguint!(1),
        Option::Some(&EquippableAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_NAME),
            slot: Slot::new_from_buffer(managed_buffer!(slot)),
        }])),
    );

    setup.assert_uris(
        EQUIPPABLE_TOKEN_ID,
        1,
        &[b"https://ipfs.io/ipfs/after custom"],
    )
}

#[test]
fn should_replace_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();

    const INIT_NONCE: u64 = 65535;

    const SHARED_SLOT: &[u8] = b"hat";
    const SHARED_ITEM_ID: &[u8] = b"ITEM-a1a1a1";

    const ITEM_TO_EQUIP_NONCE: u64 = 30;
    const ITEM_TO_EQUIP_NAME: &[u8] = b"pirate hat";

    const ITEM_TO_UNEQUIP_NAME: &[u8] = b"cap";
    const ITEM_TO_UNEQUIP_NONCE: u64 = 33u64;

    // Register hat to remove
    setup.create_equippable_with_registered_item(
        INIT_NONCE,
        SHARED_ITEM_ID,
        ITEM_TO_UNEQUIP_NONCE,
        SHARED_SLOT,
        TestItemAttributes {},
        ITEM_TO_UNEQUIP_NAME,
    );

    setup.register_and_fill_item(
        SHARED_SLOT,
        ITEM_TO_EQUIP_NAME,
        SHARED_ITEM_ID,
        ITEM_TO_EQUIP_NONCE,
        &TestItemAttributes {},
    );

    // Give the user a hat to equip
    setup.blockchain_wrapper.set_nft_balance_all_properties(
        &setup.first_user_address,
        SHARED_ITEM_ID,
        ITEM_TO_EQUIP_NONCE,
        &rust_biguint!(1),
        &TestItemAttributes {},
        0,
        Option::Some(&setup.owner_address),
        Option::Some(ITEM_TO_EQUIP_NAME),
        Option::None,
        &[],
    );

    // set CID
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes_before_custom = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::new(&[Item {
                        name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
                        slot: Slot::new_from_buffer(managed_buffer!(SHARED_SLOT)),
                    }]),
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                };

                sc.uris_of_attributes(&attributes_before_custom).set(
                    ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/cid before custom"),
                );

                let attributes_after_custom = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::new(&[Item {
                        name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_NAME),
                        slot: Slot::new_from_buffer(managed_buffer!(SHARED_SLOT)),
                    }]),
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                };

                sc.uris_of_attributes(&attributes_after_custom)
                    .set(ManagedBuffer::new_from_bytes(
                        b"https://ipfs.io/ipfs/cid after custom",
                    ));
            },
        )
        .assert_ok();

    let (esdt_transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[
        (EQUIPPABLE_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
        (
            SHARED_ITEM_ID,
            ITEM_TO_EQUIP_NONCE,
            EsdtTokenType::SemiFungible,
        ),
    ]);

    let (sc_result, tx_result) = setup.equip(esdt_transfers);

    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // Sent an equippable NFT is burned
    setup.assert_is_burn(&EQUIPPABLE_TOKEN_ID, INIT_NONCE);

    let b_wrapper = &mut setup.blockchain_wrapper;

    assert_eq!(
        b_wrapper.get_esdt_balance(
            &setup.first_user_address,
            SHARED_ITEM_ID,
            ITEM_TO_EQUIP_NONCE
        ),
        rust_biguint!(0),
        "User should not have the hat he sended"
    );

    assert_eq!(
        b_wrapper.get_esdt_balance(
            &setup.first_user_address,
            SHARED_ITEM_ID,
            ITEM_TO_UNEQUIP_NONCE
        ),
        rust_biguint!(1),
        "User should have received the hat he unequipped.",
    );

    assert_eq!(
        b_wrapper.get_esdt_balance(&setup.first_user_address, EQUIPPABLE_TOKEN_ID, 1),
        rust_biguint!(1),
        "User should have received its new equippable NFT"
    );

    // Received an equippable NFT has the right attributes
    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Option::Some(&EquippableAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_NAME),
            slot: Slot::new_from_buffer(managed_buffer!(SHARED_SLOT)),
        }])),
    );

    setup.assert_uris(
        EQUIPPABLE_TOKEN_ID,
        1,
        &[b"https://ipfs.io/ipfs/cid after custom"],
    )
}

#[test]
fn panic_if_customize_nft_without_items() {
    const INIT_NONCE: u64 = 65535;
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();
    let b_wrapper = &mut setup.blockchain_wrapper;
    // user own an equippable NFT equiped with an hat
    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &EquippableAttributes::<DebugApi>::empty(),
    );

    let (esdt_transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[(
        EQUIPPABLE_TOKEN_ID,
        INIT_NONCE,
        EsdtTokenType::NonFungible,
    )]);

    let (_, tx_result) = setup.equip(esdt_transfers);
    tx_result.assert_user_error(ERR_NEED_ONE_ITEM_OR_UNEQUIP_SLOT);
}

#[test]
fn panic_if_equip_while_nft_to_equip_is_not_an_equippable() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const INIT_NONCE: u64 = 65535;

    DebugApi::dummy();
    let b_wrapper = &mut setup.blockchain_wrapper;

    // not an equippable NFT
    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        NOT_EQUIPPABLE_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &Option::Some({}),
    );

    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        HAT_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &TestItemAttributes {},
    );

    let (esdt_transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[
        (
            NOT_EQUIPPABLE_TOKEN_ID,
            INIT_NONCE,
            EsdtTokenType::NonFungible,
        ),
        (HAT_TOKEN_ID, INIT_NONCE, EsdtTokenType::SemiFungible),
    ]);

    DebugApi::dummy();

    let (_, tx_result) = setup.equip(esdt_transfers);

    tx_result.assert_user_error(ERR_FIRST_PAYMENT_IS_EQUIPPABLE);
}

#[test]
fn panic_if_equipped_token_is_not_an_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const INIT_NONCE: u64 = 65535;
    const ITEM_TO_EQUIP_ID: &[u8] = b"NOT-AN-ITEM-a";

    DebugApi::dummy();
    let equippable_nft_attributes = EquippableAttributes::<DebugApi>::empty();

    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &equippable_nft_attributes,
    );

    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        ITEM_TO_EQUIP_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &TestItemAttributes {},
    );

    let (transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[
        (EQUIPPABLE_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
        (ITEM_TO_EQUIP_ID, INIT_NONCE, EsdtTokenType::SemiFungible),
    ]);

    let (_, tx_result) = setup.equip(transfers);

    tx_result.assert_error(
        4,
        "The item you are equipping NOT-AN-ITEM-a 65535 is not registered.",
    );
}

#[test]
fn panic_if_unequip_item_is_not_an_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();

    const INIT_NONCE: u64 = 65535;

    const SHARED_SLOT: &[u8] = b"hat";
    const SHARED_ITEM_ID: &[u8] = b"ITEM-a1a1a1";

    const ITEM_TO_EQUIP_NONCE: u64 = 30;
    const ITEM_TO_EQUIP_NAME: &[u8] = b"pirate hat";

    const ITEM_TO_UNEQUIP_NAME: &[u8] = b"cap";

    // Register penguins w/ hat to remove
    let attributes = EquippableAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
        slot: Slot::new_from_bytes(SHARED_SLOT),
    }]);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &attributes,
    );

    setup.register_and_fill_item(
        SHARED_SLOT,
        ITEM_TO_EQUIP_NAME,
        SHARED_ITEM_ID,
        ITEM_TO_EQUIP_NONCE,
        &TestItemAttributes {},
    );

    // Give the user a hat to equip
    setup.blockchain_wrapper.set_nft_balance_all_properties(
        &setup.first_user_address,
        SHARED_ITEM_ID,
        ITEM_TO_EQUIP_NONCE,
        &rust_biguint!(1),
        &TestItemAttributes {},
        0,
        Option::Some(&setup.owner_address),
        Option::Some(ITEM_TO_EQUIP_NAME),
        Option::None,
        &[],
    );

    let (esdt_transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[
        (EQUIPPABLE_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
        (
            SHARED_ITEM_ID,
            ITEM_TO_EQUIP_NONCE,
            EsdtTokenType::SemiFungible,
        ),
    ]);

    let (_, tx_result) = setup.equip(esdt_transfers);

    tx_result.assert_user_error("The item you are unequipping at slot hat is not registered.");
}

#[test]
fn test_equip_while_sending_two_as_value_of_sft() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"hat";
    const ITEM_TO_EQUIP_ID: &[u8] = b"ITEM-a1a1a1";
    const ITEM_TO_EQUIP_NAME: &[u8] = b"Pirate Hat";
    const ITEM_TO_EQUIP_NONCE: u64 = 31;
    const EQUIPPABLE_NONCE: u64 = 30;

    DebugApi::dummy();
    setup.register_and_fill_item(
        slot,
        ITEM_TO_EQUIP_NAME,
        ITEM_TO_EQUIP_ID,
        ITEM_TO_EQUIP_NONCE,
        &TestItemAttributes {},
    );

    // add empty equippable to the USER
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        EQUIPPABLE_NONCE,
        &rust_biguint!(1),
        &EquippableAttributes::<DebugApi>::empty(),
    );

    setup.add_random_item_to_user(ITEM_TO_EQUIP_ID, ITEM_TO_EQUIP_NONCE, 3);

    let transfers = vec![
        TxInputESDT {
            token_identifier: EQUIPPABLE_TOKEN_ID.to_vec(),
            nonce: EQUIPPABLE_NONCE,
            value: rust_biguint!(1),
        },
        TxInputESDT {
            token_identifier: ITEM_TO_EQUIP_ID.to_vec(),
            nonce: ITEM_TO_EQUIP_NONCE,
            value: rust_biguint!(2),
        },
    ];
    let (_, tx_result) = setup.equip(transfers);

    tx_result.assert_user_error(ERR_MORE_THAN_ONE_ITEM_RECEIVED);
}

#[test]
fn equip_while_sending_twice_same_items() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();
    const EQUIPPABLE_NONCE: u64 = 65535;
    const SLOT: &[u8] = b"hat";
    const ITEM_TO_EQUIP_ID: &[u8] = b"ITEM-a1a1a1";
    const ITEM_TO_EQUIP_NONCE: u64 = 55;
    const ITEM_TO_EQUIP_NAME: &[u8] = b"Pirate Hat";

    setup.register_and_fill_item(
        SLOT,
        ITEM_TO_EQUIP_NAME,
        ITEM_TO_EQUIP_ID,
        ITEM_TO_EQUIP_NONCE,
        &TestItemAttributes {},
    );

    setup.create_empty_equippable(EQUIPPABLE_NONCE);

    // Give the user a hat to equip
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        ITEM_TO_EQUIP_ID,
        ITEM_TO_EQUIP_NONCE,
        &rust_biguint!(2),
        &TestItemAttributes {},
    );

    // set CID
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes_before_custom = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::empty(),
                    name: ManagedBuffer::new_from_bytes(EQUIPPABLE_TOKEN_ID),
                };

                let attributes_after_custom = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::new(&[Item {
                        name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_NAME),
                        slot: Slot::new_from_bytes(SLOT),
                    }]),
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                };

                sc.uris_of_attributes(&attributes_before_custom).set(
                    &ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/cid before custom"),
                );

                sc.uris_of_attributes(&attributes_after_custom).set(
                    &ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/cid after custom"),
                );
            },
        )
        .assert_ok();

    let (esdt_transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[
        (
            EQUIPPABLE_TOKEN_ID,
            EQUIPPABLE_NONCE,
            EsdtTokenType::NonFungible,
        ),
        (
            ITEM_TO_EQUIP_ID,
            ITEM_TO_EQUIP_NONCE,
            EsdtTokenType::SemiFungible,
        ),
        (
            ITEM_TO_EQUIP_ID,
            ITEM_TO_EQUIP_NONCE,
            EsdtTokenType::SemiFungible,
        ),
    ]);

    let (sc_result, tx_result) = setup.equip(esdt_transfers);

    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // Sent an equippable NFT is burned
    setup.assert_is_burn(&EQUIPPABLE_TOKEN_ID, EQUIPPABLE_NONCE);

    let b_wrapper = &mut setup.blockchain_wrapper;

    assert_eq!(
        b_wrapper.get_esdt_balance(
            &setup.first_user_address,
            ITEM_TO_EQUIP_ID,
            ITEM_TO_EQUIP_NONCE
        ),
        rust_biguint!(1),
        "User have sent 2 items but only 1 item is equiped; so he got one back"
    );

    assert_eq!(
        b_wrapper.get_esdt_balance(&setup.first_user_address, EQUIPPABLE_TOKEN_ID, 1),
        rust_biguint!(1),
        "User should have received its new equippable NFT"
    );

    // Received an equippable NFT has the right attributes
    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Option::Some(&EquippableAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_NAME),
            slot: Slot::new_from_buffer(managed_buffer!(SLOT)),
        }])),
    );

    setup.assert_uris(
        EQUIPPABLE_TOKEN_ID,
        1,
        &[b"https://ipfs.io/ipfs/cid after custom"],
    )
}

#[test]
fn equip_while_sending_two_items_of_same_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();
    const EQUIPPABLE_NONCE: u64 = 65535;
    const SLOT: &[u8] = b"hat";
    const FIRST_ITEM_ID: &[u8] = b"FIRST-000000";
    const FIRST_ITEM_NONCE: u64 = 33;
    const FIRST_ITEM_NAME: &[u8] = b"First Item";

    const SECOND_ITEM_ID: &[u8] = b"SECOND-ffffff";
    const SECOND_ITEM_NONCE: u64 = 35;
    const SECOND_ITEM_NAME: &[u8] = b"Second Item";

    setup.create_empty_equippable(EQUIPPABLE_NONCE);

    // Give the user the first item
    setup.register_and_fill_item(
        SLOT,
        FIRST_ITEM_NAME,
        FIRST_ITEM_ID,
        FIRST_ITEM_NONCE,
        &TestItemAttributes {},
    );
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        FIRST_ITEM_ID,
        FIRST_ITEM_NONCE,
        &rust_biguint!(1),
        &TestItemAttributes {},
    );

    // Give the user the second item
    setup.register_and_fill_item(
        SLOT,
        SECOND_ITEM_NAME,
        SECOND_ITEM_ID,
        SECOND_ITEM_NONCE,
        &TestItemAttributes {},
    );
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        SECOND_ITEM_ID,
        SECOND_ITEM_NONCE,
        &rust_biguint!(1),
        &TestItemAttributes {},
    );

    // set CID
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes_before_custom = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::empty(),
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                };
                let attributes_after_custom = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::new(&[Item {
                        name: ManagedBuffer::new_from_bytes(SECOND_ITEM_NAME),
                        slot: Slot::new_from_buffer(managed_buffer!(SLOT)),
                    }]),
                    name: managed_buffer!(EQUIPPABLE_TOKEN_ID),
                };

                sc.uris_of_attributes(&attributes_before_custom).set(
                    ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/cid before custom"),
                );

                sc.uris_of_attributes(&attributes_after_custom)
                    .set(ManagedBuffer::new_from_bytes(
                        b"https://ipfs.io/ipfs/cid after custom",
                    ));
            },
        )
        .assert_ok();

    let (esdt_transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[
        (
            EQUIPPABLE_TOKEN_ID,
            EQUIPPABLE_NONCE,
            EsdtTokenType::NonFungible,
        ),
        (FIRST_ITEM_ID, FIRST_ITEM_NONCE, EsdtTokenType::SemiFungible),
        (
            SECOND_ITEM_ID,
            SECOND_ITEM_NONCE,
            EsdtTokenType::SemiFungible,
        ),
    ]);

    let (sc_result, tx_result) = setup.equip(esdt_transfers);

    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // Sent an equippable NFT is burned
    setup.assert_is_burn(&EQUIPPABLE_TOKEN_ID, EQUIPPABLE_NONCE);

    let b_wrapper = &mut setup.blockchain_wrapper;

    assert_eq!(
        b_wrapper.get_esdt_balance(&setup.first_user_address, FIRST_ITEM_ID, FIRST_ITEM_NONCE),
        rust_biguint!(1),
        "User have sent first this item but it is replaced by the second item; so he got it back"
    );

    assert_eq!(
        b_wrapper.get_esdt_balance(
            &setup.cf_wrapper.address_ref(),
            FIRST_ITEM_ID,
            FIRST_ITEM_NONCE
        ),
        rust_biguint!(2),
        "The smart contract should have sent back the first item. He own two because it need it for the attributes (like for every other items) and register_item give 2 of the items"
    );

    assert_eq!(
        b_wrapper.get_esdt_balance(&setup.first_user_address, SECOND_ITEM_ID, SECOND_ITEM_NONCE),
        rust_biguint!(0),
        "The second item has been sent to the smart contract"
    );

    assert_eq!(
        b_wrapper.get_esdt_balance(
            &setup.cf_wrapper.address_ref(),
            SECOND_ITEM_ID,
            SECOND_ITEM_NONCE
        ),
        rust_biguint!(3),
        "The smart contract received the second item; He two more because it need it for the attributes (like for every other items)"
    );

    assert_eq!(
        b_wrapper.get_esdt_balance(&setup.first_user_address, EQUIPPABLE_TOKEN_ID, 1),
        rust_biguint!(1),
        "User should have received its new equippable NFT"
    );

    // Received an equippable NFT has the right attributes
    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Option::Some(&EquippableAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(SECOND_ITEM_NAME),
            slot: Slot::new_from_buffer(managed_buffer!(SLOT)),
        }])),
    );

    setup.assert_uris(
        EQUIPPABLE_TOKEN_ID,
        1,
        &[b"https://ipfs.io/ipfs/cid after custom"],
    );
}
