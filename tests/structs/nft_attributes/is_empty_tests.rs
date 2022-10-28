use customize_nft::structs::{equippable_attributes::EquippableAttributes, item::Item, slot::Slot};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

use crate::testing_utils::New;

#[test]
fn is_empty_while_not_empty() {
    DebugApi::dummy();
    let slot = &Slot::new_from_bytes(b"hat");

    let equippable_nft_attributes = EquippableAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(b"item name"),
        slot: slot.clone(),
    }]);

    assert_eq!(equippable_nft_attributes.is_slot_empty(slot), false);
}

#[test]
fn is_empty_while_empty() {
    DebugApi::dummy();
    let slot = &Slot::new_from_bytes(b"hat");

    let equippable_nft_attributes = EquippableAttributes::<DebugApi>::empty();

    assert_eq!(equippable_nft_attributes.is_slot_empty(slot), true);
}
