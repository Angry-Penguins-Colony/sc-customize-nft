use elrond_wasm::types::{ManagedVarArgs, MultiArg2, SCResult};
use elrond_wasm_debug::{managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::item_attributes::ItemAttributes;
use equip_penguin::item_slot::ItemSlot;
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
        const ITEM_TO_EQUIP_ID: &[u8] = b"ITEM-a";

        let mut setup = utils::setup(equip_penguin::contract_obj);

        utils::set_all_permissions_on_token(&mut setup, ITEM_TO_EQUIP_ID);
        utils::register_item(&mut setup, slot.clone(), ITEM_TO_EQUIP_ID);

        let b_wrapper = &mut setup.blockchain_wrapper;

        let penguin_attributes = PenguinAttributes::<DebugApi>::empty();

        assert_eq!(penguin_attributes.is_slot_empty(&slot), Result::Ok(true));

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
            &ItemAttributes {},
        );

        let transfers = utils::create_esdt_transfers(&[
            (PENGUIN_TOKEN_ID, INIT_NONCE),
            (ITEM_TO_EQUIP_ID, INIT_NONCE),
        ]);

        b_wrapper.execute_esdt_multi_transfer(
            &setup.first_user_address,
            &setup.cf_wrapper,
            &transfers,
            |sc| {
                let managed_items_to_equip =
                    utils::create_managed_items_to_equip(&[(ITEM_TO_EQUIP_ID, INIT_NONCE)]);

                let result = sc.equip(
                    &managed_token_id!(PENGUIN_TOKEN_ID),
                    INIT_NONCE,
                    managed_items_to_equip,
                );

                utils::verbose_log_if_error(&result, "".to_string());

                assert_eq!(result, SCResult::Ok(1u64));

                StateChange::Commit
            },
        );

        // the SC don't onw the penguin
        assert_eq!(
            b_wrapper.get_esdt_balance(
                &setup.cf_wrapper.address_ref(),
                PENGUIN_TOKEN_ID,
                INIT_NONCE
            ),
            rust_biguint!(0)
        );

        // the transfered penguin has not been sent back
        assert_eq!(
            b_wrapper.get_esdt_balance(&setup.first_user_address, PENGUIN_TOKEN_ID, INIT_NONCE),
            rust_biguint!(0)
        );

        // the NEW penguin has been received
        assert_eq!(
            b_wrapper.get_esdt_balance(&setup.first_user_address, PENGUIN_TOKEN_ID, 1),
            rust_biguint!(1)
        );

        // the transfered hat has been burn
        assert_eq!(
            b_wrapper.get_esdt_balance(&setup.first_user_address, HAT_TOKEN_ID, INIT_NONCE),
            rust_biguint!(0)
        );

        b_wrapper.check_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            1u64,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::new(&[(
                slot,
                managed_token_id!(ITEM_TO_EQUIP_ID),
                INIT_NONCE,
            )]),
        );
    });
}

#[test]
fn test_equip_while_overlap() {
    utils::execute_for_all_slot(|slot| {
        const ITEM_TO_EQUIP: &[u8] = b"ITEM-a";

        let mut setup = utils::setup(equip_penguin::contract_obj);

        utils::set_all_permissions_on_token(&mut setup, ITEM_TO_EQUIP);
        utils::register_item(&mut setup, slot.clone(), ITEM_TO_EQUIP);

        let b_wrapper = &mut setup.blockchain_wrapper;
        let hat_to_remove_nonce = 56;

        // user own a penguin equiped with an hat
        b_wrapper.set_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            INIT_NONCE,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::new(&[(
                slot,
                managed_token_id!(ITEM_TO_EQUIP),
                hat_to_remove_nonce,
            )]),
        );

        let hat_to_equip_nonce = 30;
        // give the player a hat
        b_wrapper.set_nft_balance(
            &setup.first_user_address,
            ITEM_TO_EQUIP,
            hat_to_equip_nonce,
            &rust_biguint!(1),
            &ItemAttributes {},
        );

        let esdt_transfers = &utils::create_esdt_transfers(&[
            (PENGUIN_TOKEN_ID, INIT_NONCE),
            (ITEM_TO_EQUIP, hat_to_equip_nonce),
        ]);

        b_wrapper.execute_esdt_multi_transfer(
            &setup.first_user_address,
            &setup.cf_wrapper,
            esdt_transfers,
            |sc| {
                let managed_items_to_equip =
                    utils::create_managed_items_to_equip(&[(ITEM_TO_EQUIP, hat_to_equip_nonce)]);

                let result = sc.equip(
                    &TokenIdentifier::<DebugApi>::from_esdt_bytes(PENGUIN_TOKEN_ID),
                    INIT_NONCE,
                    managed_items_to_equip,
                );

                if let SCResult::Err(err) = result {
                    panic!(
                        "register_item failed: {:?}",
                        std::str::from_utf8(&err.as_bytes()).unwrap(),
                    );
                }

                assert_eq!(result, SCResult::Ok(1u64));

                StateChange::Commit
            },
        );

        // SHOULD sent removed equipment
        b_wrapper.check_nft_balance(
            &setup.first_user_address,
            ITEM_TO_EQUIP,
            INIT_NONCE,
            &rust_biguint!(1),
            &ItemAttributes {},
        );

        // SHOULD sent generated penguin
        b_wrapper.check_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            1,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::new(&[(
                slot,
                managed_token_id!(ITEM_TO_EQUIP),
                hat_to_equip_nonce,
            )]),
        );

        // sent penguin is burned
        assert_eq!(
            b_wrapper.get_esdt_balance(&setup.first_user_address, PENGUIN_TOKEN_ID, INIT_NONCE),
            rust_biguint!(0)
        );

        // new penguin is received
        assert_eq!(
            b_wrapper.get_esdt_balance(&setup.first_user_address, PENGUIN_TOKEN_ID, 1),
            rust_biguint!(1)
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
        &ItemAttributes {},
    );

    let esdt_transfers = &utils::create_esdt_transfers(&[
        (NOT_PENGUIN_TOKEN_ID, INIT_NONCE),
        (HAT_TOKEN_ID, INIT_NONCE),
    ]);

    b_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        esdt_transfers,
        |sc| {
            let managed_items_to_equip =
                utils::create_managed_items_to_equip(&[(HAT_TOKEN_ID, INIT_NONCE)]);

            let result = sc.equip(
                &TokenIdentifier::<DebugApi>::from_esdt_bytes(NOT_PENGUIN_TOKEN_ID),
                INIT_NONCE,
                managed_items_to_equip,
            );

            assert_eq!(result, SCResult::Err("Please provide a penguin".into()));

            StateChange::Commit
        },
    );
}

#[test]
fn equip_while_item_is_not_an_item() {
    utils::execute_for_all_slot(|slot| {
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
            &ItemAttributes {},
        );

        let transfers = utils::create_esdt_transfers(&[
            (PENGUIN_TOKEN_ID, INIT_NONCE),
            (ITEM_TO_EQUIP_ID, INIT_NONCE),
        ]);

        b_wrapper.execute_esdt_multi_transfer(
            &setup.first_user_address,
            &setup.cf_wrapper,
            &transfers,
            |sc| {
                let managed_items_to_equip =
                    utils::create_managed_items_to_equip(&[(ITEM_TO_EQUIP_ID, INIT_NONCE)]);

                let result = sc.equip(
                    &managed_token_id!(PENGUIN_TOKEN_ID),
                    INIT_NONCE,
                    managed_items_to_equip,
                );

                assert_eq!(
                    result,
                    SCResult::Err(
                        "You are trying to equip a token that is not considered as an item".into()
                    )
                );

                StateChange::Revert
            },
        );
    });
}
