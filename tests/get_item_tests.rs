use elrond_wasm::types::OptionalResult;
use elrond_wasm_debug::testing_framework::*;
use elrond_wasm_debug::DebugApi;
use equip_penguin::item_attributes::ItemAttributes;
use equip_penguin::*;

mod utils;

const HAT_TOKEN_ID: &[u8] = utils::HAT_TOKEN_ID;

#[test]
fn test_get_item() {
    utils::execute_for_all_slot(|slot| {
        let mut setup = utils::setup(equip_penguin::contract_obj);

        setup.register_item(slot.clone(), HAT_TOKEN_ID, &ItemAttributes::random());

        let b_wrapper = &mut setup.blockchain_wrapper;

        b_wrapper
            .execute_query(&setup.cf_wrapper, |sc| {
                let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);

                match sc.get_item_slot(&hat_token) {
                    OptionalResult::Some(item_type) => {
                        assert_eq!(item_type, slot.clone());
                    }
                    OptionalResult::None => {
                        panic!("The item is not registed, while it should be.");
                    }
                }
            })
            .assert_ok();
    });
}

#[test]
fn return_none_if_no_token_id() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let not_existing_token =
                TokenIdentifier::<DebugApi>::from_esdt_bytes("NOT_TOKEN_ID".as_bytes());

            match sc.get_item_slot(&not_existing_token) {
                OptionalResult::Some(_) => panic!("item_type found"),
                OptionalResult::None => {}
            }
        })
        .assert_ok();
}
