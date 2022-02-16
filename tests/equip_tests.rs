use elrond_wasm::contract_base::ContractBase;
use elrond_wasm::types::{EsdtTokenType, SCResult};
use elrond_wasm_debug::{managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::item::Item;
use equip_penguin::item_attributes::ItemAttributes;
use equip_penguin::penguin_attributes::PenguinAttributes;
use equip_penguin::*;

mod utils;

const PENGUIN_TOKEN_ID: &[u8] = utils::PENGUIN_TOKEN_ID;
const HAT_TOKEN_ID: &[u8] = utils::HAT_TOKEN_ID;
const NOT_PENGUIN_TOKEN_ID: &[u8] = b"QUACK-a456e";
const INIT_NONCE: u64 = 65535;

// create NFT on blockchain wrapper
#[test]
fn test_equip() {
    utils::execute_for_all_slot(|slot| {
        const ITEM_TO_EQUIP_ID: &[u8] = b"ITEM-a1a1a1";

        let mut setup = utils::setup(equip_penguin::contract_obj);

        let item_attributes = ItemAttributes::random();
        let item_init_nonce = setup.register_item(slot.clone(), ITEM_TO_EQUIP_ID, &item_attributes);

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

        let transfers = utils::create_esdt_transfers(&[
            (PENGUIN_TOKEN_ID, INIT_NONCE),
            (ITEM_TO_EQUIP_ID, item_init_nonce),
        ]);

        setup
            .blockchain_wrapper
            .execute_esdt_multi_transfer(
                &setup.first_user_address,
                &setup.cf_wrapper,
                &transfers,
                |sc| {
                    let result = sc.equip(sc.call_value().all_esdt_transfers());

                    assert_eq!(result, SCResult::Ok(1u64));

                    StateChange::Commit
                },
            )
            .assert_ok();

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
                },
            )]),
        );
    });
}

#[test]
fn test_equip_while_overlap() {
    utils::execute_for_all_slot(|slot| {
        const ITEM_TO_EQUIP: &[u8] = b"ITEM-a1a1a1";
        const HAT_TO_EQUIP_NONCE: u64 = 30;

        let mut setup = utils::setup(equip_penguin::contract_obj);

        // register hat to remove
        let hat_to_remove_nonce =
            setup.register_item(slot.clone(), ITEM_TO_EQUIP, &ItemAttributes::random());

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
                },
            )]),
        );

        // give the player a hat
        let attributes = ItemAttributes::<DebugApi>::random();
        setup.blockchain_wrapper.set_nft_balance(
            &setup.first_user_address,
            ITEM_TO_EQUIP,
            HAT_TO_EQUIP_NONCE,
            &rust_biguint!(1),
            &attributes,
        );

        let (esdt_transfers, _) = utils::create_paymens_and_esdt_transfers(&[
            (PENGUIN_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
            (
                ITEM_TO_EQUIP,
                HAT_TO_EQUIP_NONCE,
                EsdtTokenType::SemiFungible,
            ),
        ]);

        setup
            .blockchain_wrapper
            .execute_esdt_multi_transfer(
                &setup.first_user_address,
                &setup.cf_wrapper,
                &esdt_transfers,
                |sc| {
                    let result = sc.equip(sc.call_value().all_esdt_transfers());
                    utils::verbose_log_if_error(&result, "".to_string());

                    assert_eq!(result, SCResult::Ok(1u64));

                    StateChange::Commit
                },
            )
            .assert_ok();

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
    let mut setup = utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;
    // user own a penguin equiped with an hat
    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &PenguinAttributes::<DebugApi>::empty(),
    );

    let (esdt_transfers, _) = utils::create_paymens_and_esdt_transfers(&[(
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        EsdtTokenType::NonFungible,
    )]);

    b_wrapper
        .execute_esdt_multi_transfer(
            &setup.first_user_address,
            &setup.cf_wrapper,
            &esdt_transfers,
            |sc| {
                let _ = sc.equip(sc.call_value().all_esdt_transfers());

                StateChange::Revert
            },
        )
        .assert_error(4, "You must provide at least one penguin and one item.");
}

#[test]
fn equip_while_nft_to_equip_is_not_a_penguin() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

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

    let (esdt_transfers, _) = utils::create_paymens_and_esdt_transfers(&[
        (NOT_PENGUIN_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
        (HAT_TOKEN_ID, INIT_NONCE, EsdtTokenType::SemiFungible),
    ]);

    DebugApi::dummy();

    b_wrapper
        .execute_esdt_multi_transfer(
            &setup.first_user_address,
            &setup.cf_wrapper,
            &esdt_transfers,
            |sc| {
                let _ = sc.equip(sc.call_value().all_esdt_transfers());
                StateChange::Revert
            },
        )
        .assert_error(4, "Please provide a penguin as the first payment");
}

#[test]
fn equip_while_item_is_not_an_item() {
    const ITEM_TO_EQUIP_ID: &[u8] = b"NOT-AN-ITEM-a";

    let mut setup = utils::setup(equip_penguin::contract_obj);

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

    let (transfers, _) = utils::create_paymens_and_esdt_transfers(&[
        (PENGUIN_TOKEN_ID, INIT_NONCE, EsdtTokenType::NonFungible),
        (ITEM_TO_EQUIP_ID, INIT_NONCE, EsdtTokenType::SemiFungible),
    ]);

    b_wrapper
        .execute_esdt_multi_transfer(
            &setup.first_user_address,
            &setup.cf_wrapper,
            &transfers,
            |sc| {
                let _ = sc.equip(sc.call_value().all_esdt_transfers());
                StateChange::Revert
            },
        )
        .assert_error(
            4,
            "You are trying to equip a token that is not considered as an item",
        );
}

// We can test send_more_than_one_penguins because it is an NFT. And NFT doesn't have quantity.

// #[test]
// fn send_more_than_one_penguins() {
//     utils::execute_for_all_slot(|slot| {
//         const ITEM_TO_EQUIP_ID: &[u8] = b"HAT-a1a1a1";

//         let mut setup = utils::setup(equip_penguin::contract_obj);

//         setup.add_quantity(ITEM_TO_EQUIP_ID, INIT_NONCE, 1);
//         utils::set_all_permissions_on_token(&mut setup, ITEM_TO_EQUIP_ID);
//         utils::register_item(&mut setup, slot.clone(), &ITEM_TO_EQUIP_ID);

//         let b_wrapper = &mut setup.blockchain_wrapper;

//         b_wrapper.set_nft_balance(
//             &setup.first_user_address,
//             PENGUIN_TOKEN_ID,
//             INIT_NONCE,
//             &rust_biguint!(1),
//             &PenguinAttributes::<DebugApi>::empty(),
//         );

//         let transfers = vec![
//             TxInputESDT {
//                 token_identifier: PENGUIN_TOKEN_ID.to_vec(),
//                 nonce: INIT_NONCE.clone(),
//                 value: rust_biguint!(1u64),
//             },
//             TxInputESDT {
//                 token_identifier: ITEM_TO_EQUIP_ID.to_vec(),
//                 nonce: INIT_NONCE.clone(),
//                 value: rust_biguint!(2u64),
//             },
//         ];

//         b_wrapper
//             .execute_esdt_multi_transfer(
//                 &setup.first_user_address,
//                 &setup.cf_wrapper,
//                 &transfers,
//                 |sc| {
//                     sc.equip(sc.call_value().all_esdt_transfers());
//                     StateChange::Revert
//                 },
//             )
//             .assert_user_error("You must sent only one item.");
//     });
// }
