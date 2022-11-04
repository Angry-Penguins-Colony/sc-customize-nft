use customize_nft::structs::{
    equippable_attributes::{
        EquippableAttributes, ERR_NAME_CANNOT_BE_UNEQUIPPED,
        ERR_NAME_CONTAINS_UNSUPPORTED_CHARACTERS,
    },
    item::Item,
};
use elrond_wasm_debug::{managed_buffer, DebugApi};
use std::str;

use crate::testing_utils::{self, New};

#[test]
fn set_item_on_empty_slot() {
    DebugApi::dummy();

    let slot = &managed_buffer!(b"hat");

    let mut equippable_nft_attributes = EquippableAttributes::<DebugApi>::empty();

    equippable_nft_attributes.set_item_if_empty(&slot, Option::Some(managed_buffer!(b"item name")));

    let name = equippable_nft_attributes.get_name(slot).unwrap();

    assert_eq!(name, b"item name");
}

#[test]
fn set_item_on_not_empty_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_sc| {
            let slot = &managed_buffer!(b"hat");
            let mut equippable_nft_attributes = EquippableAttributes::<DebugApi>::new(&[Item {
                name: managed_buffer!(b"item name"),
                slot: slot.clone(),
            }]);

            equippable_nft_attributes
                .set_item_if_empty(&slot, Option::Some(managed_buffer!(b"item name")));
        })
        .assert_user_error("The slot is not empty. Please free it, before setting an item.");
}

#[test]
fn panic_if_name_contains_semicolon() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_sc| {
            let _ = EquippableAttributes::<DebugApi>::new(&[Item {
                name: managed_buffer!(b"item; name"),
                slot: managed_buffer!(b"hat"),
            }]);
        })
        .assert_user_error(str::from_utf8(ERR_NAME_CONTAINS_UNSUPPORTED_CHARACTERS).unwrap());
}

#[test]
fn panic_if_name_contains_colon() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_sc| {
            let _ = EquippableAttributes::<DebugApi>::new(&[Item {
                name: managed_buffer!(b"item: name"),
                slot: managed_buffer!(b"hat"),
            }]);
        })
        .assert_user_error(str::from_utf8(ERR_NAME_CONTAINS_UNSUPPORTED_CHARACTERS).unwrap());
}

#[test]
fn panic_if_name_is_unequipped() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_sc| {
            let _ = EquippableAttributes::<DebugApi>::new(&[Item {
                name: managed_buffer!(b"unequipped"),
                slot: managed_buffer!(b"hat"),
            }]);
        })
        .assert_user_error(str::from_utf8(ERR_NAME_CANNOT_BE_UNEQUIPPED).unwrap());
}
