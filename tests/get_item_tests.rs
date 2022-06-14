use customize_nft::structs::item_attributes::ItemAttributes;
use customize_nft::*;
use elrond_wasm::elrond_codec::multi_types::OptionalValue;
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::DebugApi;

mod testing_utils;

const HAT_TOKEN_ID: &[u8] = testing_utils::HAT_TOKEN_ID;

#[test]
fn test_get_item() {
    let slot = &ManagedBuffer::new_from_bytes(b"hat");
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.register_item(slot.clone(), HAT_TOKEN_ID, &ItemAttributes::random());

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);

            match sc.get_item_slot(&hat_token) {
                OptionalValue::Some(item_type) => {
                    assert_eq!(item_type, slot.clone());
                }
                OptionalValue::None => {
                    panic!("The item is not registed, while it should be.");
                }
            }
        })
        .assert_ok();
}

#[test]
fn return_none_if_no_token_id() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let not_existing_token =
                TokenIdentifier::<DebugApi>::from_esdt_bytes("NOT_TOKEN_ID".as_bytes());

            match sc.get_item_slot(&not_existing_token) {
                OptionalValue::Some(_) => panic!("item_type found"),
                OptionalValue::None => {}
            }
        })
        .assert_ok();
}
