use customize_nft::{
    libs::storage::StorageModule,
    structs::{
        equippable_nft_attributes::EquippableNftAttributes, item::Item,
        item_attributes::ItemAttributes,
    },
};
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::{rust_biguint, DebugApi};

use crate::testing_utils;

const EQUIPPABLE_TOKEN_ID: &[u8] = testing_utils::EQUIPPABLE_TOKEN_ID;

#[test]
fn customize_only_unequip() {
    // 1. ARRANGE
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"Background";
    const ITEM_TO_UNEQUIP_ID: &[u8] = b"BG-a1a1a1";
    const ITEM_TO_UNEQUIP_NAME: &[u8] = b"Some Item";
    const NONCE: u64 = 30;

    DebugApi::dummy();

    setup.create_equippable_with_registered_item(
        NONCE,
        ITEM_TO_UNEQUIP_ID,
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

                let attributes_before_custom = EquippableNftAttributes::new(&[(
                    &ManagedBuffer::new_from_bytes(slot),
                    Item {
                        token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_TO_UNEQUIP_ID),
                        nonce: NONCE,
                        name: ManagedBuffer::new_from_bytes(ITEM_TO_UNEQUIP_NAME),
                    },
                )]);

                sc.set_cid_of(
                    &attributes_before_custom,
                    ManagedBuffer::<DebugApi>::new_from_bytes(b"this is a cid"),
                );

                sc.set_cid_of(
                    &EquippableNftAttributes::<DebugApi>::empty(),
                    ManagedBuffer::new_from_bytes(b"empty"),
                );
            },
        )
        .assert_ok();

    let transfers = testing_utils::create_esdt_transfers(&[(EQUIPPABLE_TOKEN_ID, NONCE)]);

    // 2. ACT
    let (sc_result, tx_result) = setup.customize(transfers, slot);

    // 3. ASSERT
    tx_result.assert_ok();
    assert_eq!(sc_result.unwrap(), 1u64);

    // equippable & items sent burned
    setup.assert_is_burn(EQUIPPABLE_TOKEN_ID, NONCE);

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            ITEM_TO_UNEQUIP_ID,
            NONCE
        ),
        rust_biguint!(1),
        "Item unequipped should be received"
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.first_user_address,
            EQUIPPABLE_TOKEN_ID,
            1u64
        ),
        rust_biguint!(1),
        "Equippable NFT should be received"
    );

    // is equippable empty
    setup.blockchain_wrapper.check_nft_balance(
        &setup.first_user_address,
        EQUIPPABLE_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Option::Some(&EquippableNftAttributes::<DebugApi>::empty()),
    );
}
