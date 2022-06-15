use customize_nft::libs::storage::StorageModule;
use customize_nft::structs::item_attributes::ItemAttributes;
use customize_nft::*;
use elrond_wasm::types::{EsdtLocalRole, ManagedBuffer, MultiValueEncoded, TokenIdentifier};
use elrond_wasm_debug::{managed_buffer, managed_token_id};
use elrond_wasm_debug::{rust_biguint, DebugApi};

use crate::testing_utils;

#[test]
fn test_register_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";
    let slot = b"hat";

    setup.register_item(slot, TOKEN_ID, &ItemAttributes::random());

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let result = sc
                .slot_of(&TokenIdentifier::from_esdt_bytes(TOKEN_ID))
                .get();

            assert_eq!(result, managed_buffer!(slot));
        })
        .assert_ok();
}

/// Ce test vérifie que si on associe 2 items au même slot, tout fonctionne bien
#[test]
fn register_another_item_on_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const FIRST_TOKEN_ID: &[u8] = b"a";
    const SECOND_TOKEN_ID: &[u8] = b"A";

    let slot = b"slot";

    setup.register_item(slot, FIRST_TOKEN_ID, &ItemAttributes::random());
    setup.register_item(slot, SECOND_TOKEN_ID, &ItemAttributes::random());

    // We splitted check in two execute_query to avoid triggering a bug in Elrond's mocking system
    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            assert_eq!(
                sc.slot_of(&managed_token_id!(FIRST_TOKEN_ID)).get(),
                ManagedBuffer::new_from_bytes(slot)
            );
        })
        .assert_ok();

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            assert_eq!(
                sc.slot_of(&managed_token_id!(SECOND_TOKEN_ID)).get(),
                ManagedBuffer::new_from_bytes(slot)
            );
        })
        .assert_ok();
}

#[test]
fn register_unmintable_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);
    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut managed_items_ids =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                managed_items_ids.push(managed_token_id!(b"a token without minting rights"));

                let _ = sc.register_item(slot.clone(), managed_items_ids);
            },
        )
        .assert_ok();
}

#[test]
fn register_unburnable_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = &ManagedBuffer::new_from_bytes(b"hat");
    const UNBURNABLE: &[u8] = b"a token without minting rights";

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.set_esdt_local_roles(
        setup.cf_wrapper.address_ref(),
        UNBURNABLE,
        &[EsdtLocalRole::NftAddQuantity],
    );

    b_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut managed_items_ids =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                managed_items_ids.push(managed_token_id!(UNBURNABLE));

                let _ = sc.register_item(slot.clone(), managed_items_ids);
            },
        )
        .assert_ok();
}

#[test]
fn change_item_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let new_slot = b"hat";
    let old_slot = b"background";
    const ITEM_ID: &[u8] = b"ITEM-a1a1a1";

    setup.register_item(old_slot, ITEM_ID, &ItemAttributes::random());
    setup.register_item(new_slot, ITEM_ID, &ItemAttributes::random());

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let result = sc.slot_of(&managed_token_id!(ITEM_ID)).get();
            assert_eq!(result, managed_buffer!(new_slot));
        })
        .assert_ok();
}

#[test]
fn register_penguin_as_item_should_not_work() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = &ManagedBuffer::new_from_bytes(b"hat");
    const PENGUIN_TOKEN_ID: &[u8] = testing_utils::PENGUIN_TOKEN_ID;

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut managed_items_ids =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                managed_items_ids.push(managed_token_id!(PENGUIN_TOKEN_ID));

                let _ = sc.register_item(slot.clone(), managed_items_ids);
            },
        )
        .assert_error(4, "You cannot register a penguin as an item.");
}

#[test]
fn register_while_not_the_owner() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.first_user_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut managed_items_ids =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                managed_items_ids.push(managed_token_id!(b"ITEM-a1a1a1"));

                let _ = sc.register_item(slot.clone(), managed_items_ids);
            },
        )
        .assert_error(4, "Only the owner can call this method.");
}
