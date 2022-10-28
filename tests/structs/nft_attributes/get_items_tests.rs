use customize_nft::structs::{equippable_attributes::EquippableAttributes, item::Item, slot::Slot};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

use crate::testing_utils::{self, New};

#[test]
fn get_empty_item() {
    DebugApi::dummy();

    let attributes = EquippableAttributes::<DebugApi>::empty();

    let slot = Slot::<DebugApi>::new_from_bytes(b"hat");

    assert_eq!(attributes.get_name(&slot).is_none(), true);
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

        let mut attributes = EquippableAttributes::<DebugApi>::new(&[item.clone()]);

        attributes.empty_slot(slot);

        assert_eq!(attributes.get_name(&slot).is_none(), true);
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

    let attributes = EquippableAttributes::<DebugApi>::new(&[item.clone()]);

    assert_eq!(attributes.get_name(&slot).is_some(), true);
    assert_eq!(attributes.get_name(&slot).unwrap(), item.name);
}
