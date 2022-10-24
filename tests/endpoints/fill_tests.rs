use crate::testing_utils::{self, TestItemAttributes};
use customize_nft::{
    libs::storage::StorageModule,
    structs::{item::Item, slot::Slot},
    EndpointWrappers, Equip,
};
use elrond_wasm::{elrond_codec::multi_types::MultiValue4, types::MultiValueEncoded};
use elrond_wasm_debug::{managed_buffer, managed_token_id, rust_biguint};

#[test]
fn works_if_is_the_owner() {
    const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";
    const TOKEN_SLOT: &[u8] = b"hat";
    const TOKEN_NONCE: u64 = 654;
    const TOKEN_NAME: &[u8] = b"Pirate Hat";

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        &TOKEN_ID,
        TOKEN_NONCE,
        &rust_biguint!(1u64),
        &TestItemAttributes {},
    );

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.cf_wrapper,
            TOKEN_ID,
            TOKEN_NONCE,
            &rust_biguint!(1),
            |sc| {
                let mut items = MultiValueEncoded::new();
                items.push(MultiValue4::from((
                    Slot::new_from_bytes(TOKEN_SLOT),
                    managed_buffer!(TOKEN_NAME),
                    managed_token_id!(TOKEN_ID),
                    TOKEN_NONCE,
                )));

                sc.register_item(items);

                sc.call_fill();

                let opt_token = sc.get_token(&Item {
                    name: managed_buffer!(TOKEN_NAME),
                    slot: Slot::new_from_bytes(TOKEN_SLOT),
                });

                match opt_token {
                    Some(token) => {
                        assert_eq!(token.token, managed_token_id!(TOKEN_ID));
                        assert_eq!(token.nonce, TOKEN_NONCE);
                    }

                    None => {
                        panic!("Received token must be Some, but we got a None.")
                    }
                }
            },
        )
        .assert_ok();
}

#[test]
fn panic_if_not_registered() {
    const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";
    const TOKEN_NONCE: u64 = 654;

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        &TOKEN_ID,
        TOKEN_NONCE,
        &rust_biguint!(1u64),
        &TestItemAttributes {},
    );

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.cf_wrapper,
            TOKEN_ID,
            TOKEN_NONCE,
            &rust_biguint!(1),
            |sc| {
                sc.call_fill();
            },
        )
        .assert_ok();
    // .assert_user_error(ERR_CANNOT_FILL_UNREGISTERED_ITEM);
}

#[test]
fn panic_if_not_the_owner() {
    const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";
    const TOKEN_NONCE: u64 = 654;

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        &TOKEN_ID,
        TOKEN_NONCE,
        &rust_biguint!(1u64),
        &Option::Some({}),
    );

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_esdt_transfer(
            &setup.first_user_address,
            &setup.cf_wrapper,
            TOKEN_ID,
            TOKEN_NONCE,
            &rust_biguint!(1),
            |sc| {
                sc.call_fill();
            },
        )
        .assert_user_error("Endpoint can only be called by owner");
}
