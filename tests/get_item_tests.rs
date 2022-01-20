use elrond_wasm::types::OptionalResult;
use elrond_wasm_debug::testing_framework::*;
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::item_slot::ItemSlot;
use equip_penguin::*;

mod utils;

const HAT_TOKEN_ID: &[u8] = utils::utils::HAT_TOKEN_ID;

#[test]
fn test_get_item() {
    let mut setup = utils::utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);

            match sc.get_item_type(&hat_token) {
                OptionalResult::Some(item_type) => {
                    assert_eq!(item_type, ItemSlot::Hat);
                }
                OptionalResult::None => {
                    panic!("no item_type found");
                }
            }

            StateChange::Commit
        },
    );

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let not_existing_token =
                TokenIdentifier::<DebugApi>::from_esdt_bytes("PAR ALLAH PELO".as_bytes());

            match sc.get_item_type(&not_existing_token) {
                OptionalResult::Some(_) => {
                    panic!("item_type found");
                }
                OptionalResult::None => {}
            }

            StateChange::Commit
        },
    );
}
