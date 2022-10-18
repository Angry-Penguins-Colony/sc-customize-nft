use crate::testing_utils::{self, TestItemAttributes};
use customize_nft::{libs::storage::StorageModule, structs::item::Item, EndpointWrappers, Equip};
use elrond_wasm::types::{MultiValueEncoded, TokenIdentifier};
use elrond_wasm_debug::{managed_buffer, managed_token_id, rust_biguint, DebugApi};

#[test]
fn works_if_is_the_owner() {
    const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";
    const TOKEN_SLOT: &[u8] = b"hat";
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
                let mut managed_items_ids =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                managed_items_ids.push(managed_token_id!(TOKEN_ID));

                sc.register_item(managed_buffer!(TOKEN_SLOT), managed_items_ids);

                sc.call_fill();

                let (item_id, item_nonce) = sc
                    .token_of(&Item {
                        name: managed_buffer!(TOKEN_ID),
                        slot: managed_buffer!(TOKEN_SLOT),
                    })
                    .get();

                assert_eq!(item_id, managed_token_id!(TOKEN_ID));
                assert_eq!(item_nonce, TOKEN_NONCE);
            },
        )
        .assert_ok();
}

#[test]
fn panic_if_override() {
    const TOKEN_A_ID: &[u8] = b"ITEM-a1a1a1";
    const TOKEN_A_NONCE: u64 = 654;
    const TOKEN_A_SLOT: &[u8] = b"hat";

    const TOKEN_B_ID: &[u8] = TOKEN_A_ID;
    const TOKEN_B_NONCE: u64 = 1;

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        &TOKEN_A_ID,
        TOKEN_A_NONCE,
        &rust_biguint!(1u64),
        &Option::Some({}),
    );

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        &TOKEN_B_ID,
        TOKEN_B_NONCE,
        &rust_biguint!(1u64),
        &Option::Some({}),
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.cf_wrapper,
            TOKEN_A_ID,
            TOKEN_A_NONCE,
            &rust_biguint!(1),
            |sc| {
                let mut managed_items_ids =
                    MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                managed_items_ids.push(managed_token_id!(TOKEN_B_ID));

                sc.register_item(managed_buffer!(TOKEN_A_SLOT), managed_items_ids);

                sc.call_fill();

                let (item_id, item_nonce) = sc
                    .token_of(&Item {
                        name: managed_buffer!(TOKEN_A_ID),
                        slot: managed_buffer!(TOKEN_A_SLOT),
                    })
                    .get();

                assert_eq!(item_id, managed_token_id!(TOKEN_A_ID));
                assert_eq!(item_nonce, TOKEN_A_NONCE);
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.cf_wrapper,
            TOKEN_B_ID,
            TOKEN_B_NONCE,
            &rust_biguint!(1),
            |sc| {
                sc.fill();
            },
        )
        .assert_user_error(
            "The item with name ITEM-a1a1a1 is already registered. Please, use another name.",
        );
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
