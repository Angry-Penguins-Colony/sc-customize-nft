use customize_nft::{
    libs::storage::StorageModule,
    structs::{item::Item, slot::Slot, token::Token},
};
use elrond_wasm_debug::{managed_buffer, managed_token_id, rust_biguint};

use crate::testing_utils;

#[test]
fn after_insert_should_returns_valid_values() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let token = Token::new(managed_token_id!(b"HAT-a1a1a1"), 1);
                let item = Item {
                    name: managed_buffer!(b"Pirate Hat"),
                    slot: Slot::new_from_bytes(b"Hat"),
                };

                sc.map_items_tokens().insert(item.clone(), token.clone());

                assert_eq!(sc.has_item(&item), true);
                assert_eq!(sc.has_token(&token), true);

                assert_eq!(sc.get_item(&token).unwrap(), item);
                assert_eq!(sc.get_token(&item).unwrap(), token);
            },
        )
        .assert_ok();
}
