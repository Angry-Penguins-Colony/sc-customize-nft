use customize_nft::{
    libs::storage::StorageModule,
    structs::{item::Item, item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes},
};
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::testing_utils;

const PENGUIN_TOKEN_ID: &[u8] = testing_utils::PENGUIN_TOKEN_ID;

#[test]
fn customize_only_desequip() {
    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"Background";
    const ITEM_TO_DESEQUIP_ID: &[u8] = b"BG-a1a1a1";
    const ITEM_TO_DESEQUIP_NAME: &[u8] = b"Some Item";
    const NONCE: u64 = 30;

    DebugApi::dummy();

    setup.create_penguin_with_registered_item(
        NONCE,
        ITEM_TO_DESEQUIP_ID,
        NONCE,
        slot,
        ItemAttributes::random(),
    );

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.ipfs_gateway()
                    .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/"));

                let attributes = PenguinAttributes::new(&[(
                    &ManagedBuffer::new_from_bytes(slot),
                    Item {
                        token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_TO_DESEQUIP_ID),
                        nonce: NONCE,
                        name: ManagedBuffer::new_from_bytes(ITEM_TO_DESEQUIP_NAME),
                    },
                )]);

                sc.set_thumbnail_cid(
                    &attributes,
                    ManagedBuffer::<DebugApi>::new_from_bytes(b"this is a cid"),
                );
            },
        )
        .assert_ok();

    let transfers = testing_utils::create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE)]);

    // 2. ACT
    let (sc_result, tx_result) = setup.customize(transfers, managed_buffer!(slot));

    // 3. ASSERT
    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // penguin&items sent burned
    setup.assert_is_burn(PENGUIN_TOKEN_ID, NONCE);

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            ITEM_TO_DESEQUIP_ID,
            NONCE
        ),
        rust_biguint!(1),
        "Item desequipped should be received"
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            1u64
        ),
        rust_biguint!(1),
        "Penguin should be received"
    );

    // is pinguin empty
    setup.blockchain_wrapper.check_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Option::Some(&PenguinAttributes::<DebugApi>::empty()),
    );
}
