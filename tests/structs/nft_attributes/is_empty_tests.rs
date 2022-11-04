use customize_nft::structs::{equippable_attributes::EquippableAttributes, item::Item};
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::testing_utils::New;

#[test]
fn is_empty_while_not_empty() {
    DebugApi::dummy();
    let slot = &managed_buffer!(b"hat");

    let equippable_nft_attributes = EquippableAttributes::<DebugApi>::new(&[Item {
        name: managed_buffer!(b"item name"),
        slot: slot.clone(),
    }]);

    assert_eq!(equippable_nft_attributes.is_slot_empty(slot), false);
}

#[test]
fn is_empty_while_empty() {
    DebugApi::dummy();
    let slot = &managed_buffer!(b"hat");

    let equippable_nft_attributes = EquippableAttributes::<DebugApi>::empty();

    assert_eq!(equippable_nft_attributes.is_slot_empty(slot), true);
}
