use customize_nft::constants::{
    ERR_CANNOT_OVERRIDE_REGISTERED_ITEM, ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM,
};
use customize_nft::libs::storage::StorageModule;
use customize_nft::structs::item::Item;
use customize_nft::structs::slot::Slot;
use customize_nft::structs::token::Token;
use customize_nft::*;
use elrond_wasm::elrond_codec::multi_types::MultiValue4;
use elrond_wasm::types::{MultiValueEncoded, TokenIdentifier};
use elrond_wasm_debug::{managed_buffer, managed_token_id};
use elrond_wasm_debug::{rust_biguint, DebugApi};

use crate::testing_utils::{self, TestItemAttributes};

#[test]
fn test_register_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"hat";
    const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";
    const TOKEN_NONCE: u64 = 42;
    const ITEM_NAME: &[u8] = b"Pirate Hat";

    DebugApi::dummy();

    setup.register_and_fill_item(
        slot,
        ITEM_NAME,
        TOKEN_ID,
        TOKEN_NONCE,
        &TestItemAttributes {},
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let result = sc.get_item(&Token::new(
                TokenIdentifier::from_esdt_bytes(TOKEN_ID),
                TOKEN_NONCE,
            ));

            assert_eq!(result.is_some(), true);
            assert_eq!(
                result.unwrap(),
                Item {
                    slot: Slot::new_from_buffer(managed_buffer!(slot)),
                    name: managed_buffer!(ITEM_NAME)
                }
            );
        })
        .assert_ok();
}

/// Ce test vérifie que si on associe 2 items au même slot, tout fonctionne bien
#[test]
fn register_another_item_on_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const FIRST_TOKEN_ID: &[u8] = b"a";
    const FIRST_TOKEN_NONCE: u64 = 42;
    const FIRST_ITEM_NAME: &[u8] = b"first item";

    const SECOND_TOKEN_ID: &[u8] = b"A";
    const SECOND_TOKEN_NONCE: u64 = 43;
    const SECOND_ITEM_NAME: &[u8] = b"second item";

    const COMMON_SLOT: &[u8] = b"slot";

    DebugApi::dummy();
    setup.register_and_fill_item(
        COMMON_SLOT,
        FIRST_ITEM_NAME,
        FIRST_TOKEN_ID,
        FIRST_TOKEN_NONCE,
        &TestItemAttributes {},
    );
    setup.register_and_fill_item(
        COMMON_SLOT,
        SECOND_ITEM_NAME,
        SECOND_TOKEN_ID,
        SECOND_TOKEN_NONCE,
        &TestItemAttributes {},
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            assert_eq!(
                sc.get_item(&Token::new(
                    managed_token_id!(FIRST_TOKEN_ID),
                    FIRST_TOKEN_NONCE
                ))
                .unwrap(),
                Item {
                    slot: Slot::new_from_bytes(COMMON_SLOT),
                    name: managed_buffer!(FIRST_ITEM_NAME)
                }
            );

            assert_eq!(
                sc.get_item(&Token::new(
                    managed_token_id!(SECOND_TOKEN_ID),
                    SECOND_TOKEN_NONCE
                ))
                .unwrap(),
                Item {
                    slot: Slot::new_from_bytes(COMMON_SLOT),
                    name: managed_buffer!(SECOND_ITEM_NAME)
                }
            );
        })
        .assert_ok();
}

#[test]
fn panic_if_override() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const TOKEN_ID: &[u8] = b"HAT-a1a1a1";
    const TOKEN_NONCE: u64 = 1;

    let first_slot = b"hat";
    let first_slot_item_name = b"pirate hat";

    let second_slot = b"clothes";
    let second_slot_item_name = b"Golden Chain";

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut items = MultiValueEncoded::new();
                items.push(MultiValue4::from((
                    Slot::new_from_bytes(first_slot),
                    managed_buffer!(first_slot_item_name),
                    managed_token_id!(TOKEN_ID),
                    TOKEN_NONCE,
                )));

                sc.register_item(items);
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut items = MultiValueEncoded::new();
                items.push(MultiValue4::from((
                    Slot::new_from_bytes(second_slot),
                    managed_buffer!(second_slot_item_name),
                    managed_token_id!(TOKEN_ID),
                    TOKEN_NONCE,
                )));

                sc.register_item(items);
            },
        )
        .assert_user_error(ERR_CANNOT_OVERRIDE_REGISTERED_ITEM);
}

#[test]
fn panic_if_register_equippable() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"hat";

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut items = MultiValueEncoded::new();
                items.push(MultiValue4::from((
                    Slot::new_from_bytes(slot),
                    managed_buffer!(b"My Equippable"),
                    managed_token_id!(testing_utils::EQUIPPABLE_TOKEN_ID),
                    1,
                )));

                sc.register_item(items);
            },
        )
        .assert_user_error(ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM);
}

#[test]
fn panic_if_not_the_owner() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.first_user_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.call_register_item();
            },
        )
        .assert_user_error("Endpoint can only be called by owner");
}
