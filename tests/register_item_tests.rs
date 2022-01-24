use elrond_wasm::types::{
    EsdtLocalRole, ManagedVarArgs, ManagedVec, SCError, SCResult, StaticSCError,
};
use elrond_wasm_debug::{managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::item_slot::ItemSlot;
use equip_penguin::*;

mod utils;

const HAT_TOKEN_ID: &[u8] = utils::HAT_TOKEN_ID;
const ANOTHER_HAT_TOKEN_ID: &[u8] = utils::HAT_2_TOKEN_ID;

#[test]
fn test_register_item() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    utils::register_item(&mut setup, ItemSlot::Hat, HAT_TOKEN_ID);
    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let managed_token_id = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);
            let mut managed_items_ids = ManagedVec::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id.clone());

            let result = sc.items_slot(&managed_token_id!(HAT_TOKEN_ID)).get();

            assert_eq!(result, ItemSlot::Hat);

            StateChange::Commit
        },
    );
}

/// Ce test vérifie que si on associe 2 items au même slot, tout fonctionne bien
#[test]
fn register_another_item_on_slot() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    utils::register_item(&mut setup, ItemSlot::Hat, HAT_TOKEN_ID);
    utils::register_item(&mut setup, ItemSlot::Hat, ANOTHER_HAT_TOKEN_ID);
    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_query(&setup.cf_wrapper, |sc| {
        let result = sc.items_slot(&managed_token_id!(HAT_TOKEN_ID)).get();

        assert_eq!(result, ItemSlot::Hat);

        let result2 = sc
            .items_slot(&managed_token_id!(ANOTHER_HAT_TOKEN_ID))
            .get();

        assert_eq!(result2, ItemSlot::Hat);
    });
}

#[test]
fn register_unmintable_item() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0),
        |sc| {
            let mut managed_items_ids =
                ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id!(b"a token without minting rights"));

            let result = sc.register_item(ItemSlot::Hat, managed_items_ids);

            assert_eq!(result, SCResult::Err("Local mint role not set".into()));

            StateChange::Revert
        },
    );
}

#[test]
fn register_unburnable_item() {
    const UNBURNABLE: &[u8] = b"a token without minting rights";

    let mut setup = utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.set_esdt_local_roles(
        setup.cf_wrapper.address_ref(),
        UNBURNABLE,
        &[EsdtLocalRole::Mint],
    );

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0),
        |sc| {
            let mut managed_items_ids =
                ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id!(UNBURNABLE));

            let result = sc.register_item(ItemSlot::Hat, managed_items_ids);

            assert_eq!(result, SCResult::Err("Local burn role not set".into()));

            StateChange::Revert
        },
    );
}

#[test]
fn change_item_slot() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    const ITEM_ID: &[u8] = HAT_TOKEN_ID;

    utils::register_item(&mut setup, ItemSlot::Hat, ITEM_ID);
    utils::register_item(&mut setup, ItemSlot::Background, ITEM_ID);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_query(&setup.cf_wrapper, |sc| {
        let result = sc.items_slot(&managed_token_id!(ITEM_ID)).get();
        assert_eq!(result, ItemSlot::Background);
    });
}

#[test]
fn register_penguin_as_item_should_not_work() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    const PENGUIN_TOKEN_ID: &[u8] = utils::PENGUIN_TOKEN_ID;

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let mut managed_items_ids =
                ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id!(PENGUIN_TOKEN_ID));

            let result = sc.register_item(ItemSlot::Hat, managed_items_ids);
            assert_eq!(
                result,
                SCResult::Err(StaticSCError::from(
                    &b"You cannot register a penguin as an item."[..]
                ))
            );

            StateChange::Revert
        },
    );
}

#[test]
fn register_while_not_the_owner() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.first_user_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let mut managed_items_ids =
                ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id!(HAT_TOKEN_ID));

            let result = sc.register_item(ItemSlot::Hat, managed_items_ids);
            assert_eq!(
                result,
                SCResult::Err(StaticSCError::from(
                    &b"Only the owner can call this method."[..]
                ))
            );

            StateChange::Revert
        },
    );
}
