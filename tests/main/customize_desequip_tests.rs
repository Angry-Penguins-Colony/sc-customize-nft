use customize_nft::{
    libs::storage::StorageModule,
    structs::{item::Item, item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes},
};
use elrond_wasm::types::{ManagedBuffer, SCResult, TokenIdentifier};
use elrond_wasm_debug::{rust_biguint, DebugApi};

use crate::testing_utils;

const PENGUIN_TOKEN_ID: &[u8] = testing_utils::PENGUIN_TOKEN_ID;

#[test]
fn customize_only_desequip() {
    DebugApi::dummy();

    let slot = ManagedBuffer::new_from_bytes(b"Background");

    const ITEM_TO_DESEQUIP_ID: &[u8] = b"BG-a1a1a1";
    const NONCE: u64 = 30;

    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.create_penguin_with_registered_item(
        NONCE,
        ITEM_TO_DESEQUIP_ID,
        NONCE,
        slot.clone(),
        ItemAttributes::random(),
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            sc.ipfs_gateway()
                .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/"));

            let attributes = PenguinAttributes::new(&[(
                &slot,
                Item {
                    token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_TO_DESEQUIP_ID),
                    nonce: NONCE,
                    name: ManagedBuffer::new_from_bytes(b"item name"),
                },
            )]);

            sc.set_thumbnail_cid(
                &attributes,
                ManagedBuffer::<DebugApi>::new_from_bytes(b"this is a cid"),
            );
        })
        .assert_ok();

    let transfers = testing_utils::create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE)]);

    // 2. ACT
    let (sc_result, tx_result) = setup.customize(transfers, slot.clone());

    // 3. ASSERT
    tx_result.assert_ok();
    assert_eq!(sc_result, SCResult::Ok(1u64));

    // penguin&items sent burned
    setup.assert_is_burn(PENGUIN_TOKEN_ID, NONCE);

    // item desequipped received
    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            ITEM_TO_DESEQUIP_ID,
            NONCE
        ),
        rust_biguint!(1)
    );

    // new desquiped penguin received
    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            1u64
        ),
        rust_biguint!(1)
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
