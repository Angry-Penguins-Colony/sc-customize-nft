use elrond_wasm::types::{EsdtTokenType, ManagedBuffer, SCResult};
use elrond_wasm_debug::managed_token_id;
use elrond_wasm_debug::tx_mock::TxInputESDT;
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::structs::item::Item;
use equip_penguin::structs::item_attributes::ItemAttributes;
use equip_penguin::structs::penguin_attributes::PenguinAttributes;

mod testing_utils;

const PENGUIN_TOKEN_ID: &[u8] = testing_utils::PENGUIN_TOKEN_ID;
const HAT_TOKEN_ID: &[u8] = testing_utils::HAT_TOKEN_ID;
const NOT_PENGUIN_TOKEN_ID: &[u8] = b"QUACK-a456e";
const INIT_NONCE: u64 = 65535;

// create NFT on blockchain wrapper
#[test]
fn test_equip() {
    testing_utils::execute_for_all_slot(|slot| {
        const ITEM_TO_EQUIP_ID: &[u8] = b"ITEM-a1a1a1";
        const ITEM_TO_EQUIP_NAME: &[u8] = b"item name";

        let mut setup = testing_utils::setup(equip_penguin::contract_obj);

        let item_attributes = ItemAttributes::random();
        let item_init_nonce = setup.register_item_all_properties(
            slot.clone(),
            ITEM_TO_EQUIP_ID,
            &item_attributes,
            0,
            Option::None,
            Option::Some(ITEM_TO_EQUIP_NAME),
            Option::None,
            &[],
        );

        // add empty pingouin to the USER
        setup.blockchain_wrapper.set_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            INIT_NONCE,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::empty(),
        );

        setup.blockchain_wrapper.set_nft_balance(
            &setup.first_user_address,
            ITEM_TO_EQUIP_ID,
            item_init_nonce,
            &rust_biguint!(1),
            &item_attributes,
        );

        let transfers = testing_utils::create_esdt_transfers(&[
            (PENGUIN_TOKEN_ID, INIT_NONCE),
            (ITEM_TO_EQUIP_ID, item_init_nonce),
        ]);

        let (sc_result, tx_result) = setup.equip(transfers);

        tx_result.assert_ok();
        assert_eq!(sc_result, SCResult::Ok(1u64));

        // the transfered penguin is burn
        setup.assert_is_burn(&PENGUIN_TOKEN_ID, INIT_NONCE);
        setup.assert_is_burn(&HAT_TOKEN_ID, item_init_nonce);

        // the NEW penguin has been received
        assert_eq!(
            setup.blockchain_wrapper.get_esdt_balance(
                &setup.first_user_address,
                PENGUIN_TOKEN_ID,
                1
            ),
            rust_biguint!(1)
        );

        // the NEW penguin has the right attributes
        setup.blockchain_wrapper.check_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            1u64,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::new(&[(
                slot,
                Item {
                    token: managed_token_id!(ITEM_TO_EQUIP_ID),
                    nonce: item_init_nonce,
                    name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_NAME),
                },
            )]),
        );
    });
}

#[test]
fn test_equip_while_overlap() {
    testing_utils::execute_for_all_slot(|slot| {
        const ITEM_TO_EQUIP: &[u8] = b"ITEM-a1a1a1";
        const HAT_TO_EQUIP_NONCE: u64 = 30;
        const OLD_HAT_NAME: &[u8] = b"old hat";
        const NEW_HAT_NAME: &[u8] = b"new hat";

        let mut setup = testing_utils::setup(equip_penguin::contract_obj);

        // register hat to remove
        let hat_to_remove_nonce = setup.register_item_all_properties(
            slot.clone(),
            ITEM_TO_EQUIP,
            &ItemAttributes::random(),
            0u64,
            Option::None,
            Option::Some(OLD_HAT_NAME),
            Option::None,
            &[],
        );

        // user own a penguin equiped with an hat
        setup.blockchain_wrapper.set_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            INIT_NONCE,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::new(&[(
                slot,
                Item {
                    token: managed_token_id!(ITEM_TO_EQUIP),
                    nonce: hat_to_remove_nonce,
                    name: ManagedBuffer::new_from_bytes(NEW_HAT_NAME),
                },
            )]),
        );

        // give the player a hat
        let attributes = ItemAttributes::<DebugApi>::random();
        setup.blockchain_wrapper.set_nft_balance_all_properties(
            &setup.first_user_address,
            ITEM_TO_EQUIP,
            HAT_TO_EQUIP_NONCE,
            &rust_biguint!(1),
            &attributes,
            0,
            Option::Some(&setup.owner_address),
            Option::Some(NEW_HAT_NAME),
            Option::None,
            &[],
        );

        setup.blockchain_wrapper.set_nft_balance_all_properties(
            &setup.first_user_address,
            ITEM_TO_EQUIP,
            HAT_TO_EQUIP_NONCE,
            &rust_biguint!(1),
            &attributes,
            0,
            Option::Some(&setup.owner_address),
            Option::Some(NEW_HAT_NAME),
            Option::None,
            &[],
        );

        let (esdt_transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[
            (PENGUIN_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
            (
                ITEM_TO_EQUIP,
                HAT_TO_EQUIP_NONCE,
                EsdtTokenType::SemiFungible,
            ),
        ]);

        let (sc_result, tx_result) = setup.equip(esdt_transfers);

        tx_result.assert_ok();
        assert_eq!(sc_result, SCResult::Ok(1u64));

        // sent penguin is burned
        setup.assert_is_burn(&PENGUIN_TOKEN_ID, INIT_NONCE);

        let b_wrapper = &mut setup.blockchain_wrapper;

        // sent removed equipment
        assert_eq!(
            b_wrapper.get_esdt_balance(&setup.first_user_address, ITEM_TO_EQUIP, INIT_NONCE),
            rust_biguint!(1)
        );

        // new penguin is received
        assert_eq!(
            b_wrapper.get_esdt_balance(&setup.first_user_address, PENGUIN_TOKEN_ID, 1),
            rust_biguint!(1)
        );

        // NEW penguin has the right attributes
        b_wrapper.check_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            1,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::new(&[(
                slot,
                Item {
                    token: managed_token_id!(ITEM_TO_EQUIP),
                    nonce: HAT_TO_EQUIP_NONCE,
                    name: ManagedBuffer::new_from_bytes(b"new hat"),
                },
            )]),
        );

        // previously hat is sent
        assert_eq!(
            b_wrapper.get_esdt_balance(
                &setup.first_user_address,
                ITEM_TO_EQUIP,
                hat_to_remove_nonce
            ),
            rust_biguint!(1)
        );
    });
}

#[test]
fn equip_penguin_without_items() {
    let mut setup = testing_utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;
    // user own a penguin equiped with an hat
    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &PenguinAttributes::<DebugApi>::empty(),
    );

    let (esdt_transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[(
        PENGUIN_TOKEN_ID,
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
fn equip_while_nft_to_equip_is_not_a_penguin() {
    let mut setup = testing_utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    // not a penguin
    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        NOT_PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &{},
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
        (NOT_PENGUIN_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
        (HAT_TOKEN_ID, INIT_NONCE, EsdtTokenType::SemiFungible),
    ]);

    DebugApi::dummy();

    let (_, tx_result) = setup.equip(esdt_transfers);

    tx_result.assert_user_error("Please provide a penguin as the first payment");
}

#[test]
fn equip_while_item_is_not_an_item() {
    const ITEM_TO_EQUIP_ID: &[u8] = b"NOT-AN-ITEM-a";

    let mut setup = testing_utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    let penguin_attributes = PenguinAttributes::<DebugApi>::empty();

    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &penguin_attributes,
    );

    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        ITEM_TO_EQUIP_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &ItemAttributes {
            item_id: ManagedBuffer::<DebugApi>::new(),
        },
    );

    let (transfers, _) = testing_utils::create_paymens_and_esdt_transfers(&[
        (PENGUIN_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
        (ITEM_TO_EQUIP_ID, INIT_NONCE, EsdtTokenType::SemiFungible),
    ]);

    let (_, tx_result) = setup.equip(transfers);

    tx_result.assert_error(
        4,
        "You are trying to equip a token that is not considered as an item",
    );
}

#[test]
fn test_equip_while_sending_two_as_value_of_sft() {
    testing_utils::execute_for_all_slot(|slot| {
        const ITEM_TO_EQUIP_ID: &[u8] = b"ITEM-a1a1a1";
        const NONCE: u64 = 30;

        let mut setup = testing_utils::setup(equip_penguin::contract_obj);

        setup.register_item(slot.clone(), ITEM_TO_EQUIP_ID, &ItemAttributes::random());

        // add empty pingouin to the USER
        setup.blockchain_wrapper.set_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            NONCE,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::empty(),
        );

        setup.add_random_item_to_user(ITEM_TO_EQUIP_ID, NONCE, 3);

        let transfers = vec![
            TxInputESDT {
                token_identifier: PENGUIN_TOKEN_ID.to_vec(),
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

        tx_result.assert_user_error("You must sent only one item.");
    });
}
