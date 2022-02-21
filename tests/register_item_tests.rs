use elrond_wasm::types::{EsdtLocalRole, ManagedVarArgs, ManagedVec, SCResult};
use elrond_wasm_debug::{managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::libs::storage::StorageModule;
use equip_penguin::structs::item_attributes::ItemAttributes;
use equip_penguin::structs::item_slot::ItemSlot;
use equip_penguin::*;

mod utils;

const HAT_TOKEN_ID: &[u8] = utils::HAT_TOKEN_ID;
const ANOTHER_HAT_TOKEN_ID: &[u8] = utils::HAT_2_TOKEN_ID;

#[test]
fn test_register_item() {
    utils::execute_for_all_slot(|slot| {
        const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";

        let mut setup = utils::setup(equip_penguin::contract_obj);

        setup.register_item(slot.clone(), TOKEN_ID, &ItemAttributes::random());

        let b_wrapper = &mut setup.blockchain_wrapper;

        b_wrapper
            .execute_tx(
                &setup.owner_address,
                &setup.cf_wrapper,
                &rust_biguint!(0u64),
                |sc| {
                    let managed_token_id =
                        TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);
                    let mut managed_items_ids =
                        ManagedVec::<DebugApi, TokenIdentifier<DebugApi>>::new();
                    managed_items_ids.push(managed_token_id.clone());

                    let result = sc.items_slot(&managed_token_id!(TOKEN_ID)).get();

                    assert_eq!(&result, slot);

                    StateChange::Commit
                },
            )
            .assert_ok();
    });
}

/// Ce test vérifie que si on associe 2 items au même slot, tout fonctionne bien
#[test]
fn register_another_item_on_slot() {
    utils::execute_for_all_slot(|slot| {
        let mut setup = utils::setup(equip_penguin::contract_obj);

        setup.register_item(slot.clone(), HAT_TOKEN_ID, &ItemAttributes::random());
        setup.register_item(
            slot.clone(),
            ANOTHER_HAT_TOKEN_ID,
            &ItemAttributes::random(),
        );

        let b_wrapper = &mut setup.blockchain_wrapper;

        b_wrapper
            .execute_query(&setup.cf_wrapper, |sc| {
                let result = sc.items_slot(&managed_token_id!(HAT_TOKEN_ID)).get();

                assert_eq!(&result, slot);

                let result2 = sc
                    .items_slot(&managed_token_id!(ANOTHER_HAT_TOKEN_ID))
                    .get();

                assert_eq!(&result2, slot);
            })
            .assert_ok();
    });
}

#[test]
fn register_unmintable_item() {
    utils::execute_for_all_slot(|slot| {
        let mut setup = utils::setup(equip_penguin::contract_obj);

        let b_wrapper = &mut setup.blockchain_wrapper;

        b_wrapper
            .execute_tx(
                &setup.owner_address,
                &setup.cf_wrapper,
                &rust_biguint!(0),
                |sc| {
                    let mut managed_items_ids =
                        ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
                    managed_items_ids.push(managed_token_id!(b"a token without minting rights"));

                    let _ = sc.register_item(slot.clone(), managed_items_ids);

                    StateChange::Revert
                },
            )
            .assert_error(4, "Local add quantity role not set for an item");
    });
}

#[test]
fn register_unburnable_item() {
    utils::execute_for_all_slot(|slot| {
        const UNBURNABLE: &[u8] = b"a token without minting rights";

        let mut setup = utils::setup(equip_penguin::contract_obj);

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
                        ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
                    managed_items_ids.push(managed_token_id!(UNBURNABLE));

                    let result = sc.register_item(slot.clone(), managed_items_ids);

                    assert_eq!(
                        result,
                        SCResult::Err(("Local burn role not set for an item").into())
                    );

                    StateChange::Revert
                },
            )
            .assert_error(4, "Local burn role not set for an item");
    });
}

#[test]
fn change_item_slot() {
    utils::execute_for_all_slot(|new_slot| {
        const ITEM_ID: &[u8] = HAT_TOKEN_ID;
        const OLD_SLOT: ItemSlot = ItemSlot::Hat;

        let mut setup = utils::setup(equip_penguin::contract_obj);

        setup.register_item(OLD_SLOT.clone(), ITEM_ID, &ItemAttributes::random());
        setup.register_item(new_slot.clone(), ITEM_ID, &ItemAttributes::random());

        let b_wrapper = &mut setup.blockchain_wrapper;

        b_wrapper
            .execute_query(&setup.cf_wrapper, |sc| {
                let result = sc.items_slot(&managed_token_id!(ITEM_ID)).get();
                assert_eq!(&result, &new_slot.clone());
            })
            .assert_ok();
    });
}

#[test]
fn register_penguin_as_item_should_not_work() {
    utils::execute_for_all_slot(|slot| {
        let mut setup = utils::setup(equip_penguin::contract_obj);

        const PENGUIN_TOKEN_ID: &[u8] = utils::PENGUIN_TOKEN_ID;

        let b_wrapper = &mut setup.blockchain_wrapper;

        b_wrapper
            .execute_tx(
                &setup.owner_address,
                &setup.cf_wrapper,
                &rust_biguint!(0u64),
                |sc| {
                    let mut managed_items_ids =
                        ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
                    managed_items_ids.push(managed_token_id!(PENGUIN_TOKEN_ID));

                    let _ = sc.register_item(slot.clone(), managed_items_ids);

                    StateChange::Revert
                },
            )
            .assert_error(4, "You cannot register a penguin as an item.");
    });
}

#[test]
fn register_while_not_the_owner() {
    utils::execute_for_all_slot(|slot| {
        let mut setup = utils::setup(equip_penguin::contract_obj);

        let b_wrapper = &mut setup.blockchain_wrapper;

        b_wrapper
            .execute_tx(
                &setup.first_user_address,
                &setup.cf_wrapper,
                &rust_biguint!(0u64),
                |sc| {
                    let mut managed_items_ids =
                        ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
                    managed_items_ids.push(managed_token_id!(HAT_TOKEN_ID));

                    let _ = sc.register_item(slot.clone(), managed_items_ids);

                    StateChange::Revert
                },
            )
            .assert_error(4, "Only the owner can call this method.");
    });
}
