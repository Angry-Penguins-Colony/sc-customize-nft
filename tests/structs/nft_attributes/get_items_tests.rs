use customize_nft::structs::{
    equippable_nft_attributes::EquippableNftAttributes, item::Item, slot::Slot,
};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

use crate::testing_utils;

#[test]
fn get_empty_item() {
    DebugApi::dummy();

    let attributes = EquippableNftAttributes::<DebugApi>::empty();

    let slot = Slot::<DebugApi>::new_from_bytes(b"hat");

    assert_eq!(attributes.get_item(&slot).is_none(), true);
}

#[test]
fn should_return_none_if_emptied() {
    let setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.execute_in_managed_environment(|| {
        let slot = &Slot::new_from_bytes(b"hat");
        let name = &ManagedBuffer::<DebugApi>::new_from_bytes(b"pirate hat");

        let item = Item {
            slot: slot.clone(),
            name: name.clone(),
        };

        let mut attributes = EquippableNftAttributes::<DebugApi>::new(&[item.clone()]);

        attributes.empty_slot(slot);

        assert_eq!(attributes.get_item(&slot).is_none(), true);
    })
}

#[test]
fn should_return_some_with_slot_different_case() {
    let setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.execute_in_managed_environment(|| {
        let registered_slot = &Slot::new_from_bytes(b"hat");
        let query_slot = &Slot::new_from_bytes(b"HAT");
        let name = &ManagedBuffer::<DebugApi>::new_from_bytes(b"pirate hat");

        let item = Item {
            slot: registered_slot.clone(),
            name: name.clone(),
        };

        let attributes = EquippableNftAttributes::<DebugApi>::new(&[item.clone()]);

        assert_eq!(attributes.get_item(&query_slot).is_some(), true);
        assert_eq!(attributes.get_item(&query_slot).unwrap(), item);
    })
}

#[test]
fn get_item() {
    DebugApi::dummy();

    let slot = Slot::new_from_bytes(b"hat");
    let item = Item {
        name: ManagedBuffer::<DebugApi>::new_from_bytes(b"hat"),
        slot: slot.clone(),
    };

    let attributes = EquippableNftAttributes::<DebugApi>::new(&[item.clone()]);

    assert_eq!(attributes.get_item(&slot).is_some(), true);
    assert_eq!(attributes.get_item(&slot).unwrap(), item);
}
