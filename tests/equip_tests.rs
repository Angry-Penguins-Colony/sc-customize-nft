use elrond_wasm::types::{ManagedVarArgs, MultiArg2, SCResult};
use elrond_wasm_debug::{managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::item_attributes::ItemAttributes;
use equip_penguin::item_slot::ItemSlot;
use equip_penguin::penguins_attributes::PenguinAttributes;
use equip_penguin::*;

mod utils;

const PENGUIN_TOKEN_ID: &[u8] = utils::utils::PENGUIN_TOKEN_ID;
const HAT_TOKEN_ID: &[u8] = utils::utils::HAT_TOKEN_ID;
const NOT_PENGUIN_TOKEN_ID: &[u8] = b"QUACK-a456e";
const INIT_NONCE: u64 = 65535;

// create NFT on blockchain wrapper
#[test]
fn test_equip() {
    let mut setup = utils::utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    let none_value = (TokenIdentifier::<DebugApi>::from_esdt_bytes(b""), 0);

    let penguin_attributes = PenguinAttributes {
        hat: none_value.clone(),
    };

    assert_eq!(
        penguin_attributes.is_slot_empty(&ItemSlot::Hat),
        Result::Ok(true)
    );

    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &penguin_attributes,
    );

    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        HAT_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &ItemAttributes {},
    );

    let transfers = utils::utils::create_esdt_transfers(&[
        (PENGUIN_TOKEN_ID, INIT_NONCE),
        (HAT_TOKEN_ID, INIT_NONCE),
    ]);

    b_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        &transfers,
        |sc| {
            let mut managed_items_to_equip =
                ManagedVarArgs::<DebugApi, MultiArg2<TokenIdentifier<DebugApi>, u64>>::new();
            managed_items_to_equip.push(MultiArg2((managed_token_id!(HAT_TOKEN_ID), INIT_NONCE)));

            let result = sc.equip(
                &managed_token_id!(PENGUIN_TOKEN_ID),
                INIT_NONCE,
                managed_items_to_equip,
            );

            assert_eq!(result, SCResult::Ok(1u64));

            StateChange::Commit
        },
    );

    // generated penguin has been sent
    b_wrapper.check_nft_balance(
        &setup.cf_wrapper.address_ref(),
        PENGUIN_TOKEN_ID,
        1u64,
        &rust_biguint!(0),
        &PenguinAttributes {
            hat: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
                INIT_NONCE,
            ),
        },
    );

    // the transfered penguin has been burn
    b_wrapper.check_nft_balance(
        &setup.cf_wrapper.address_ref(),
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(0),
        &PenguinAttributes {
            hat: none_value.clone(),
        },
    );

    // the transfered penguin has not been sent back
    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(0),
        &PenguinAttributes {
            hat: none_value.clone(),
        },
    );

    // the NEW penguin has been received
    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        1u64,
        &rust_biguint!(1),
        &PenguinAttributes {
            hat: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
                INIT_NONCE,
            ),
        },
    );

    // the transfered hat has been burn
    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        HAT_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(0),
        &ItemAttributes {},
    );
}

#[test]
fn test_equip_while_overlap() {
    let mut setup = utils::utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    let hat_to_remove_nonce = 56;

    // user own a penguin equiped with an hat
    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &PenguinAttributes {
            hat: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
                hat_to_remove_nonce,
            ),
        },
    );

    let hat_to_equip_nonce = 30;
    // give the player a hat
    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        HAT_TOKEN_ID,
        hat_to_equip_nonce,
        &rust_biguint!(1),
        &ItemAttributes {},
    );

    let esdt_transfers = &utils::utils::create_esdt_transfers(&[
        (PENGUIN_TOKEN_ID, INIT_NONCE),
        (HAT_TOKEN_ID, hat_to_equip_nonce),
    ]);

    b_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        esdt_transfers,
        |sc| {
            let mut managed_items_to_equip =
                ManagedVarArgs::<DebugApi, MultiArg2<TokenIdentifier<DebugApi>, u64>>::new();
            managed_items_to_equip.push(MultiArg2((
                TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
                hat_to_equip_nonce,
            )));

            let result = sc.equip(
                &TokenIdentifier::<DebugApi>::from_esdt_bytes(PENGUIN_TOKEN_ID),
                INIT_NONCE,
                managed_items_to_equip,
            );

            assert_eq!(result, SCResult::Ok(1u64));

            StateChange::Commit
        },
    );

    // SHOULD sent removed equipment
    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        HAT_TOKEN_ID,
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
        &PenguinAttributes {
            hat: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
                hat_to_equip_nonce,
            ),
        },
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
        b_wrapper.get_esdt_balance(&setup.first_user_address, HAT_TOKEN_ID, hat_to_remove_nonce),
        rust_biguint!(1)
    );
}

#[test]
fn equip_while_nft_to_equip_is_not_a_penguin() {
    let mut setup = utils::utils::setup(equip_penguin::contract_obj);

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

    let esdt_transfers = &utils::utils::create_esdt_transfers(&[
        (NOT_PENGUIN_TOKEN_ID, INIT_NONCE),
        (HAT_TOKEN_ID, INIT_NONCE),
    ]);

    b_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        esdt_transfers,
        |sc| {
            let mut managed_items_to_equip =
                ManagedVarArgs::<DebugApi, MultiArg2<TokenIdentifier<DebugApi>, u64>>::new();
            managed_items_to_equip.push(MultiArg2((
                TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
                INIT_NONCE,
            )));

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
