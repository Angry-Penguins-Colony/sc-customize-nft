use customize_nft::{
    libs::storage::StorageModule,
    structs::{item::Item, item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes},
    Equip,
};
use elrond_wasm::{
    contract_base::ContractBase,
    types::{ManagedBuffer, MultiValueEncoded, TokenIdentifier},
};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::testing_utils;

const PENGUIN_TOKEN_ID: &[u8] = testing_utils::PENGUIN_TOKEN_ID;

#[test]
fn customize_complete_flow() {
    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const PENGUIN_NONCE: u64 = 5;

    const ITEM_TO_DESEQUIP_SLOT: &[u8] = b"background";
    const ITEM_TO_DESEQUIP_ID: &[u8] = b"ITEM-a1a1a1";
    const ITEM_TO_DESEQUIP_NONCE: u64 = 30;

    const ITEM_TO_EQUIP_SLOT: &[u8] = b"hat";
    const ITEM_TO_EQUIP_ID: &[u8] = b"HAT-b2b2b2";

    DebugApi::dummy();

    // Create penguin with item to desequip
    setup.create_penguin_with_registered_item(
        PENGUIN_NONCE,
        ITEM_TO_DESEQUIP_ID,
        ITEM_TO_DESEQUIP_NONCE,
        ITEM_TO_DESEQUIP_SLOT,
        ItemAttributes::random(),
    );

    // Register item to equip
    let item_to_equip_nonce = setup.register_item(
        ITEM_TO_EQUIP_SLOT,
        ITEM_TO_EQUIP_ID,
        &ItemAttributes::random(),
    );

    // Add to user an item to equip
    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        ITEM_TO_EQUIP_ID,
        item_to_equip_nonce,
        &rust_biguint!(1),
        &ItemAttributes::<DebugApi>::random(),
    );

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes_before_custom = PenguinAttributes::new(&[(
                    &ManagedBuffer::new_from_bytes(ITEM_TO_DESEQUIP_SLOT),
                    Item {
                        token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_TO_DESEQUIP_ID),
                        nonce: ITEM_TO_DESEQUIP_NONCE,
                        name: ManagedBuffer::new_from_bytes(ITEM_TO_DESEQUIP_ID),
                    },
                )]);

                let attributes_after_custom = PenguinAttributes::new(&[(
                    &ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_SLOT),
                    Item {
                        token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_TO_EQUIP_ID),
                        nonce: item_to_equip_nonce,
                        name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_ID),
                    },
                )]);

                sc.set_cid_of(
                    &attributes_before_custom,
                    ManagedBuffer::new_from_bytes(b"cid before custom"),
                );

                sc.set_cid_of(
                    &attributes_after_custom,
                    ManagedBuffer::new_from_bytes(b"cid after custom"),
                );
            },
        )
        .assert_ok();

    let transfers = testing_utils::create_esdt_transfers(&[
        (PENGUIN_TOKEN_ID, PENGUIN_NONCE),
        (ITEM_TO_EQUIP_ID, item_to_equip_nonce),
    ]);

    // 2. ACT
    let (sc_result, tx_result) = setup.customize(transfers, ITEM_TO_DESEQUIP_SLOT);

    // 3. ASSERT
    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // penguin sent burned
    setup.assert_is_burn(PENGUIN_TOKEN_ID, item_to_equip_nonce);

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.cf_wrapper.address_ref(),
            ITEM_TO_EQUIP_ID,
            item_to_equip_nonce
        ),
        rust_biguint!(3),
        "The user should have send the item to equip on the smart contract + the 2 items from register_item() function."
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            ITEM_TO_DESEQUIP_ID,
            ITEM_TO_DESEQUIP_NONCE
        ),
        rust_biguint!(1),
        "The user should have received the item desequipped"
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            1u64
        ),
        rust_biguint!(1),
        "The user should have received the penguin"
    );

    setup.blockchain_wrapper.check_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Option::Some(&PenguinAttributes::<DebugApi>::new(&[(
            &managed_buffer!(ITEM_TO_EQUIP_SLOT),
            Item {
                token: TokenIdentifier::from_esdt_bytes(ITEM_TO_EQUIP_ID),
                nonce: item_to_equip_nonce,
                name: ManagedBuffer::new_from_bytes(ITEM_TO_EQUIP_ID), // the name should be ITEM_TO_EQUIP_NAME but a bug in rust testing framework force us to do this
            },
        )])),
    );
}

#[test]
fn customize_nothing_to_desequip_and_equip() {
    const NONCE: u64 = 30;

    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    DebugApi::dummy();
    setup.create_penguin_empty(NONCE);

    let transfers = testing_utils::create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE)]);

    // 2. ACT
    let tx_result = setup.blockchain_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        &transfers,
        |sc| {
            let managed_slots = MultiValueEncoded::<DebugApi, ManagedBuffer<DebugApi>>::new();

            let _ = sc.customize(sc.call_value().all_esdt_transfers(), managed_slots);
        },
    );

    // 3. ASSERT
    tx_result.assert_user_error(
        "You must either provide at least one penguin and one item OR provide a slot to desequip.",
    );
}
