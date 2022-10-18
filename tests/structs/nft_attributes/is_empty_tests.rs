use customize_nft::structs::{
    equippable_nft_attributes::EquippableNftAttributes, item::Item, slot::Slot,
};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

use crate::testing_utils::{self, New};

#[test]
fn is_empty_while_not_empty() {
    DebugApi::dummy();
    let slot = &Slot::new_from_bytes(b"hat");

    let equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(b"item name"),
        slot: slot.clone(),
    }]);

    assert_eq!(equippable_nft_attributes.is_slot_empty(slot), false);
}

#[test]
fn is_empty_while_empty() {
    DebugApi::dummy();
    let slot = &Slot::new_from_bytes(b"hat");

    let equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::empty();

    assert_eq!(equippable_nft_attributes.is_slot_empty(slot), true);
}

#[test]
fn should_return_is_empty_with_different_case() {
    let setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.execute_in_managed_environment(|| {
        let slot_lowercase = &Slot::new_from_bytes(b"hat");
        let slot_uppercase = &Slot::new_from_bytes(b"HAT");

        let equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(b"item name"),
            slot: slot_lowercase.clone(),
        }]);

        assert_eq!(
            equippable_nft_attributes.is_slot_empty(slot_uppercase),
            false
        );
    });
}
