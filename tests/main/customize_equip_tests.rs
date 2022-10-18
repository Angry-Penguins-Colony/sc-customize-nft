use crate::testing_utils::TestItemAttributes;
use customize_nft::constants::{
    ERR_FIRST_PAYMENT_IS_EQUIPPABLE, ERR_MORE_THAN_ONE_ITEM_RECEIVED,
    ERR_NEED_ONE_ITEM_OR_UNEQUIP_SLOT,
};
use customize_nft::libs::storage::StorageModule;
use customize_nft::structs::equippable_nft_attributes::EquippableNftAttributes;
use customize_nft::structs::item::Item;
use elrond_wasm::contract_base::ContractBase;
use elrond_wasm::elrond_codec::multi_types::MultiValue2;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm::types::{EsdtTokenType, ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::managed_buffer;
use elrond_wasm_debug::tx_mock::TxInputESDT;
use elrond_wasm_debug::{rust_biguint, DebugApi};

use crate::{args_set_cid_of, testing_utils};

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
        ITEM_TO_EQUIP_ID,
        ITEM_TO_EQUIP_NONCE,
        &TestItemAttributes {},
        0,
        Option::None,
        Option::Some(ITEM_TO_EQUIP_NAME),
        Option::None,
        &[],
    );

    // PRINTING name of ITEM_TO_EQUIP
    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let data = sc.blockchain().get_esdt_token_data(
                &sc.blockchain().get_sc_address(),
                &TokenIdentifier::from_esdt_bytes(ITEM_TO_EQUIP_ID),
                ITEM_TO_EQUIP_NONCE,
            );

            println!("{:?}", data.name);
        })
        .assert_ok();

    // add empty equippable to the USER
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        EQUIPPABLE_TOKEN_NONCE,
        &rust_biguint!(1),
        &EquippableNftAttributes::<DebugApi>::empty(),
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
                let attributes_before_custom = EquippableNftAttributes::<DebugApi>::empty();

                let attributes_after_custom = EquippableNftAttributes::<DebugApi>::new(&[Item {
                    name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework force us to do this
                    slot: managed_buffer!(slot),
                }]);

                sc.set_cid_of(args_set_cid_of!(
                    attributes_before_custom,
                    ManagedBuffer::new_from_bytes(b"cid before custom")
                ));

                sc.set_cid_of(args_set_cid_of!(
                    attributes_after_custom,
                    ManagedBuffer::new_from_bytes(b"after custom")
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
        Option::Some(&EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework force us to do this
            slot: managed_buffer!(slot),
        }])),
    );

    setup.assert_uris(
        EQUIPPABLE_TOKEN_ID,
        1,
        &[b"https://ipfs.io/ipfs/after custom"],
    )
}

#[test]
fn equip_item_while_another_item_equipped_on_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();

    const INIT_NONCE: u64 = 65535;
    const ITEM_ID: &[u8] = b"ITEM-a1a1a1";

    const ITEM_TO_EQUIP_NONCE: u64 = 30;
    const ITEM_TO_EQUIP_NAME: &[u8] = b"pirate hat";

    const ITEM_TO_UNEQUIP_NAME: &[u8] = b"cap";
    const ITEM_TO_UNEQUIP_NONCE: u64 = 33u64;

    let slot = b"hat";

    // Register hat to remove
    setup.register_and_fill_items_all_properties(
        slot,
        ITEM_ID,
        ITEM_TO_UNEQUIP_NONCE,
        &TestItemAttributes {},
        0u64,
        Option::None,
        Option::Some(ITEM_TO_UNEQUIP_NAME),
        Option::None,
        &[],
    );

    // Give the user an equippable NFT equiped with the hat to remove
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(ITEM_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in Elrond mocking force us to do this
            slot: managed_buffer!(slot),
        }]),
    );

    // Give the user a hat to equip
    setup.blockchain_wrapper.set_nft_balance_all_properties(
        &setup.first_user_address,
        ITEM_ID,
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
                let attributes_before_custom = EquippableNftAttributes::<DebugApi>::new(&[Item {
                    name: ManagedBuffer::new_from_bytes(ITEM_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in Elrond mocking force us to do this
                    slot: managed_buffer!(slot),
                }]);

                let attributes_after_custom = EquippableNftAttributes::<DebugApi>::new(&[Item {
                    name: ManagedBuffer::new_from_bytes(ITEM_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework 0.32.0 force us to do this
                    slot: managed_buffer!(slot),
                }]);

                sc.set_cid_of(args_set_cid_of!(
                    attributes_before_custom,
                    ManagedBuffer::new_from_bytes(b"cid before custom")
                ));

                sc.set_cid_of(args_set_cid_of!(
                    attributes_after_custom,
                    ManagedBuffer::new_from_bytes(b"cid after custom")
                ));
            },
        )
        .assert_ok();

    let (esdt_transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[
        (EQUIPPABLE_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
        (ITEM_ID, ITEM_TO_EQUIP_NONCE, EsdtTokenType::SemiFungible),
    ]);

    let (sc_result, tx_result) = setup.equip(esdt_transfers);

    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // Sent an equippable NFT is burned
    setup.assert_is_burn(&EQUIPPABLE_TOKEN_ID, INIT_NONCE);

    let b_wrapper = &mut setup.blockchain_wrapper;

    assert_eq!(
        b_wrapper.get_esdt_balance(&setup.first_user_address, ITEM_ID, ITEM_TO_EQUIP_NONCE),
        rust_biguint!(0),
        "User should not have the hat he sended"
    );

    assert_eq!(
        b_wrapper.get_esdt_balance(&setup.first_user_address, ITEM_ID, ITEM_TO_UNEQUIP_NONCE),
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
        Option::Some(&EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(ITEM_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework 0.32.0 force us to do this
            slot: managed_buffer!(slot),
        }])),
    );

    setup.assert_uris(
        EQUIPPABLE_TOKEN_ID,
        1,
        &[b"https://ipfs.io/ipfs/cid after custom"],
    )
}

#[test]
fn customize_nft_without_items() {
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
        &EquippableNftAttributes::<DebugApi>::empty(),
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
fn equip_while_nft_to_equip_is_not_an_equippable() {
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
fn panic_if_token_is_not_an_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const INIT_NONCE: u64 = 65535;
    const ITEM_TO_EQUIP_ID: &[u8] = b"NOT-AN-ITEM-a";

    DebugApi::dummy();
    let equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::empty();

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

    tx_result.assert_error(4, "No slot found for NOT-AN-ITEM-a.");
}

#[test]
fn test_equip_while_sending_two_as_value_of_sft() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"hat";
    const ITEM_TO_EQUIP_ID: &[u8] = b"ITEM-a1a1a1";
    const ITEM_TO_EQUIP_NONCE: u64 = 31;
    const EQUIPPABLE_NONCE: u64 = 30;

    DebugApi::dummy();
    setup.register_and_fill_item(
        slot,
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
        &EquippableNftAttributes::<DebugApi>::empty(),
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

    setup.register_and_fill_item(
        SLOT,
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
                let attributes_before_custom = EquippableNftAttributes::<DebugApi>::empty();

                let attributes_after_custom = EquippableNftAttributes::<DebugApi>::new(&[Item {
                    name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework 0.32.0 force us to do this
                    slot: managed_buffer!(SLOT),
                }]);

                sc.set_cid_of(args_set_cid_of!(
                    attributes_before_custom,
                    ManagedBuffer::new_from_bytes(b"cid before custom")
                ));

                sc.set_cid_of(args_set_cid_of!(
                    attributes_after_custom,
                    ManagedBuffer::new_from_bytes(b"cid after custom")
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
        Option::Some(&EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework 0.32.0 force us to do this
            slot: managed_buffer!(SLOT),
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
    const SECOND_ITEM_ID: &[u8] = b"SECOND-ffffff";
    const SECOND_ITEM_NONCE: u64 = 35;

    setup.create_empty_equippable(EQUIPPABLE_NONCE);

    // Give the user the first item
    setup.register_and_fill_item(
        SLOT,
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
                let attributes_after_custom = EquippableNftAttributes::<DebugApi>::new(&[Item {
                    name: ManagedBuffer::new_from_bytes(SECOND_ITEM_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework 0.32.0 force us to do this
                    slot: managed_buffer!(SLOT),
                }]);

                sc.set_cid_of(args_set_cid_of!(
                    attributes_after_custom,
                    ManagedBuffer::new_from_bytes(b"cid after custom")
                ));

                sc.set_cid_of(args_set_cid_of!(
                    EquippableNftAttributes::<DebugApi>::empty(),
                    ManagedBuffer::new_from_bytes(b"cid before custom")
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
        Option::Some(&EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(SECOND_ITEM_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework 0.32.0 force us to do this
            slot: managed_buffer!(SLOT),
        }])),
    );

    setup.assert_uris(
        EQUIPPABLE_TOKEN_ID,
        1,
        &[b"https://ipfs.io/ipfs/cid after custom"],
    );
}
