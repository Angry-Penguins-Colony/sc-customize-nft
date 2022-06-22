use customize_nft::constants::{ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM, ERR_NOT_OWNER};
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

    let slot = b"hat";
    const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";
    const TOKEN_NONCE: u64 = 42;

    DebugApi::dummy();

    setup.register_and_fill_item(
        slot,
        TOKEN_ID,
        TOKEN_NONCE,
        &ItemAttributes::<DebugApi>::random(),
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let result = sc
                .__slot_of(&TokenIdentifier::from_esdt_bytes(TOKEN_ID))
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
    const FIRST_TOKEN_NONCE: u64 = 42;
    const SECOND_TOKEN_ID: &[u8] = b"A";
    const SECOND_TOKEN_NONCE: u64 = 43;

    let slot = b"slot";

    DebugApi::dummy();
    setup.register_and_fill_item(
        slot,
        FIRST_TOKEN_ID,
        FIRST_TOKEN_NONCE,
        &ItemAttributes::random(),
    );
    setup.register_and_fill_item(
        slot,
        SECOND_TOKEN_ID,
        SECOND_TOKEN_NONCE,
        &ItemAttributes::random(),
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            assert_eq!(
                sc.__slot_of(&managed_token_id!(FIRST_TOKEN_ID)).get(),
                ManagedBuffer::new_from_bytes(slot)
            );
            assert_eq!(
                sc.__slot_of(&managed_token_id!(SECOND_TOKEN_ID)).get(),
                ManagedBuffer::new_from_bytes(slot)
            );
        })
        .assert_ok();
}

#[test]
fn register_unmintable_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let slot = ManagedBuffer::new_from_bytes(b"hat");

                let mut managed_items_ids =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                managed_items_ids.push(managed_token_id!(b"a token without minting rights"));

                let _ = sc.register_item(slot, managed_items_ids);
            },
        )
        .assert_ok();
}

#[test]
fn register_unburnable_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"hat";
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

                let _ = sc.register_item(ManagedBuffer::new_from_bytes(slot), managed_items_ids);
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
    const ITEM_NONCE: u64 = 42;

    DebugApi::dummy();
    setup.register_and_fill_item(old_slot, ITEM_ID, ITEM_NONCE, &ItemAttributes::random());
    setup.register_and_fill_item(new_slot, ITEM_ID, ITEM_NONCE, &ItemAttributes::random());

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let result = sc.__slot_of(&managed_token_id!(ITEM_ID)).get();
            assert_eq!(result, managed_buffer!(new_slot));
        })
        .assert_ok();
}

#[test]
fn register_to_slot_with_different_case() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let first_slot = b"hat";
    let first_slot_item = b"pirate hat";

    let second_slot = b"Hat";
    let second_slot_item = b"Cap";

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut first_slot_items =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                first_slot_items.push(managed_token_id!(first_slot_item));

                sc.register_item(ManagedBuffer::new_from_bytes(first_slot), first_slot_items);

                let mut second_slot_items =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                second_slot_items.push(managed_token_id!(second_slot_item));
                sc.register_item(
                    ManagedBuffer::new_from_bytes(second_slot),
                    second_slot_items,
                );

                assert_eq!(
                    sc.get_slot_of(&managed_token_id!(second_slot_item)),
                    managed_buffer!(&second_slot.to_ascii_lowercase())
                );
                assert_eq!(
                    sc.get_slot_of(&managed_token_id!(first_slot_item)),
                    managed_buffer!(&first_slot.to_ascii_lowercase())
                );
            },
        )
        .assert_ok();
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
                let mut managed_items_ids =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                managed_items_ids.push(managed_token_id!(testing_utils::EQUIPPABLE_TOKEN_ID));

                let _ = sc.register_item(ManagedBuffer::new_from_bytes(slot), managed_items_ids);
            },
        )
        .assert_user_error(ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM);
}

#[test]
fn panic_if_not_the_owner() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"hat";

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

                let _ = sc.register_item(ManagedBuffer::new_from_bytes(slot), managed_items_ids);
            },
        )
        .assert_user_error(ERR_NOT_OWNER);
}
