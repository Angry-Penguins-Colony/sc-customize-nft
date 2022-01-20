use elrond_wasm::types::ManagedVec;
use elrond_wasm_debug::testing_framework::*;
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::item_slot::ItemSlot;
use equip_penguin::*;

mod utils;

const HAT_TOKEN_ID: &[u8] = utils::utils::HAT_TOKEN_ID;

#[test]
fn test_register_item() {
    let mut setup = utils::utils::setup(equip_penguin::contract_obj);

    utils::utils::register_item(&mut setup, ItemSlot::Hat, HAT_TOKEN_ID);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let managed_token_id = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);
            let mut managed_items_ids = ManagedVec::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id.clone());

            match sc.items_types().get(&ItemSlot::Hat) {
                Some(output_items) => {
                    assert_eq!(output_items, managed_items_ids);
                }
                None => {
                    panic!("no item_type found");
                }
            }

            StateChange::Commit
        },
    );
}
