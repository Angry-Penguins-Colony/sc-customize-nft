use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

use crate::testing_utils;

#[test]
fn set_item_on_empty_slot() {
    DebugApi::dummy();

    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::empty();

    equippable_nft_attributes.set_item_if_empty(
        &slot,
        Option::Some(Item {
            name: ManagedBuffer::new_from_bytes(b"item name"),
            slot: slot.clone(),
        }),
    );

    let item = equippable_nft_attributes.get_item(slot).unwrap();

    assert_eq!(item.name, b"item name");
}

#[test]
fn set_item_on_not_empty_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_sc| {
            let slot = &ManagedBuffer::new_from_bytes(b"hat");
            let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[Item {
                name: ManagedBuffer::new_from_bytes(b"item name"),
                slot: slot.clone(),
            }]);

            equippable_nft_attributes.set_item_if_empty(
                &slot,
                Option::Some(Item {
                    name: ManagedBuffer::new_from_bytes(b"item name"),
                    slot: slot.clone(),
                }),
            );
        })
        .assert_user_error("The slot is not empty. Please free it, before setting an item.");
}
