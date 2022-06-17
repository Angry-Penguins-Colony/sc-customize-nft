use customize_nft::constants::{ERR_FIRST_PAYMENT_IS_EQUIPPABLE, ERR_MORE_THAN_ONE_ITEM_RECEIVED};
use customize_nft::libs::storage::StorageModule;
use customize_nft::structs::equippable_nft_attributes::EquippableNftAttributes;
use customize_nft::structs::item::Item;
use customize_nft::structs::item_attributes::ItemAttributes;
use elrond_wasm::contract_base::ContractBase;
use elrond_wasm::types::{EsdtTokenType, ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::tx_mock::TxInputESDT;
use elrond_wasm_debug::{managed_buffer, managed_token_id};
use elrond_wasm_debug::{rust_biguint, DebugApi};

use crate::testing_utils;

const EQUIPPABLE_TOKEN_ID: &[u8] = testing_utils::EQUIPPABLE_TOKEN_ID;
const HAT_TOKEN_ID: &[u8] = testing_utils::HAT_TOKEN_ID;
const NOT_EQUIPPABLE_TOKEN_ID: &[u8] = b"QUACK-a456e";
const INIT_NONCE: u64 = 65535;

// create NFT on blockchain wrapper
#[test]
fn test_equip() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();

    let slot = b"hat";
    const ITEM_TO_EQUIP_ID: &[u8] = b"ITEM-a1a1a1";
    const ITEM_TO_EQUIP_NAME: &[u8] = b"item name";

    let item_attributes = ItemAttributes::random();
    let item_nonce = setup.register_item_all_properties(
        slot,
        ITEM_TO_EQUIP_ID,
        &item_attributes,
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
                item_nonce,
            );

            println!("{:?}", data.name);
        })
        .assert_ok();

    // add empty pingouin to the USER
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &EquippableNftAttributes::<DebugApi>::empty(),
    );

    // add item_to_equip_id
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        ITEM_TO_EQUIP_ID,
        item_nonce,
        &rust_biguint!(1),
        &item_attributes,
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

                let attributes_after_custom = EquippableNftAttributes::<DebugApi>::new(&[(
                    &managed_buffer!(slot),
                    Item {
                        token: managed_token_id!(ITEM_TO_EQUIP_ID),
                        nonce: item_nonce,
                        name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework force us to do this
                    },
                )]);

                sc.set_cid_of(
                    &attributes_before_custom,
                    ManagedBuffer::new_from_bytes(b"cid before custom"),
                );

                sc.set_cid_of(
                    &attributes_after_custom,
                    ManagedBuffer::new_from_bytes(b"after custom"),
                );
            },
        )
        .assert_ok();

    let transfers = testing_utils::create_esdt_transfers(&[
        (EQUIPPABLE_TOKEN_ID, INIT_NONCE),
        (ITEM_TO_EQUIP_ID, item_nonce),
    ]);

    let (sc_result, tx_result) = setup.equip(transfers);

    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // the transfered equippable is burn
    setup.assert_is_burn(&EQUIPPABLE_TOKEN_ID, INIT_NONCE);
    setup.assert_is_burn(&HAT_TOKEN_ID, item_nonce);

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
        Option::Some(&EquippableNftAttributes::<DebugApi>::new(&[(
            &managed_buffer!(slot),
            Item {
                token: managed_token_id!(ITEM_TO_EQUIP_ID),
                nonce: item_nonce,
                name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework force us to do this
            },
        )])),
    );
}

#[test]
fn equip_item_while_another_item_equipped_on_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();

    const ITEM_ID: &[u8] = b"ITEM-a1a1a1";

    const ITEM_TO_EQUIP_NONCE: u64 = 30;
    const ITEM_TO_EQUIP_NAME: &[u8] = b"pirate hat";

    const ITEM_TO_DESEQUIP_NAME: &[u8] = b"cap";

    let slot = b"hat";

    // Register hat to remove
    let item_to_desequip_nonce = setup.register_item_all_properties(
        slot,
        ITEM_ID,
        &ItemAttributes::<DebugApi>::random(),
        0u64,
        Option::None,
        Option::Some(ITEM_TO_DESEQUIP_NAME),
        Option::None,
        &[],
    );

    // Give the user an equippable NFT equiped with the hat to remove
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &EquippableNftAttributes::<DebugApi>::new(&[(
            &managed_buffer!(slot),
            Item {
                token: managed_token_id!(ITEM_ID),
                nonce: item_to_desequip_nonce,
                name: ManagedBuffer::new_from_bytes(ITEM_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in Elrond mocking force us to do this
            },
        )]),
    );

    // Give the user a hat to equip
    setup.blockchain_wrapper.set_nft_balance_all_properties(
        &setup.first_user_address,
        ITEM_ID,
        ITEM_TO_EQUIP_NONCE,
        &rust_biguint!(1),
        &ItemAttributes::<DebugApi>::random(),
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
                let attributes_before_custom = EquippableNftAttributes::<DebugApi>::new(&[(
                    &managed_buffer!(slot),
                    Item {
                        token: managed_token_id!(ITEM_ID),
                        nonce: item_to_desequip_nonce,
                        name: ManagedBuffer::new_from_bytes(ITEM_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in Elrond mocking force us to do this
                    },
                )]);

                let attributes_after_custom = EquippableNftAttributes::<DebugApi>::new(&[(
                    &managed_buffer!(slot),
                    Item {
                        token: managed_token_id!(ITEM_ID),
                        nonce: ITEM_TO_EQUIP_NONCE,
                        name: ManagedBuffer::new_from_bytes(ITEM_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework 0.32.0 force us to do this
                    },
                )]);

                sc.set_cid_of(
                    &attributes_before_custom,
                    ManagedBuffer::new_from_bytes(b"cid before custom"),
                );

                sc.set_cid_of(
                    &attributes_after_custom,
                    ManagedBuffer::new_from_bytes(b"cid after custom"),
                );
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
        b_wrapper.get_esdt_balance(&setup.first_user_address, ITEM_ID, item_to_desequip_nonce),
        rust_biguint!(1),
        "User should have received the hat he desequipped.",
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
        Option::Some(&EquippableNftAttributes::<DebugApi>::new(&[(
            &managed_buffer!(slot),
            Item {
                token: managed_token_id!(ITEM_ID),
                nonce: ITEM_TO_EQUIP_NONCE,
                name: ManagedBuffer::new_from_bytes(ITEM_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework 0.32.0 force us to do this
            },
        )])),
    );
}

#[test]
fn customize_nft_without_items() {
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
    tx_result.assert_error(
        4,
        "You must either provide at least one penguin and one item OR provide a slot to desequip.",
    );
}

#[test]
fn equip_while_nft_to_equip_is_not_an_equippable() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

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
        &ItemAttributes {
            item_id: ManagedBuffer::<DebugApi>::new(),
        },
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
fn equip_while_item_is_not_an_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

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
        &ItemAttributes {
            item_id: ManagedBuffer::<DebugApi>::new(),
        },
    );

    let (transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[
        (EQUIPPABLE_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
        (ITEM_TO_EQUIP_ID, INIT_NONCE, EsdtTokenType::SemiFungible),
    ]);

    let (_, tx_result) = setup.equip(transfers);

    tx_result.assert_error(
        4,
        "Trying to equip NOT-AN-ITEM-a but is not considered as an item",
    );
}

#[test]
fn test_equip_while_sending_two_as_value_of_sft() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"hat";
    const ITEM_TO_EQUIP_ID: &[u8] = b"ITEM-a1a1a1";
    const NONCE: u64 = 30;

    DebugApi::dummy();
    setup.register_item(slot, ITEM_TO_EQUIP_ID, &ItemAttributes::random());

    // add empty pingouin to the USER
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        NONCE,
        &rust_biguint!(1),
        &EquippableNftAttributes::<DebugApi>::empty(),
    );

    setup.add_random_item_to_user(ITEM_TO_EQUIP_ID, NONCE, 3);

    let transfers = vec![
        TxInputESDT {
            token_identifier: EQUIPPABLE_TOKEN_ID.to_vec(),
            nonce: NONCE,
            value: rust_biguint!(1),
        },
        TxInputESDT {
            token_identifier: ITEM_TO_EQUIP_ID.to_vec(),
            nonce: NONCE,
            value: rust_biguint!(2),
        },
    ];
    let (_, tx_result) = setup.equip(transfers);

    tx_result.assert_user_error(ERR_MORE_THAN_ONE_ITEM_RECEIVED);
}
