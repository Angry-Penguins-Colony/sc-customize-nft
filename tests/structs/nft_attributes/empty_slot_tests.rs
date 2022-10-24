use customize_nft::structs::{
    equippable_nft_attributes::EquippableNftAttributes, item::Item, slot::Slot,
};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::testing_utils::New;

#[test]
fn should_empty_slot_if_already_empty() {
    DebugApi::dummy();

    const SLOT: &[u8] = b"hat";

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[]);

    equippable_nft_attributes.empty_slot(&Slot::new_from_buffer(managed_buffer!(SLOT)));

    assert_eq!(
        equippable_nft_attributes.is_slot_empty(&Slot::new_from_buffer(managed_buffer!(SLOT))),
        true
    );
}

#[test]
fn should_empty_slot() {
    DebugApi::dummy();

    const SLOT: &[u8] = b"hat";

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(b"item name"),
        slot: Slot::new_from_buffer(managed_buffer!(SLOT)),
    }]);

    equippable_nft_attributes.empty_slot(&Slot::new_from_buffer(managed_buffer!(SLOT)));
    assert_eq!(
        equippable_nft_attributes.is_slot_empty(&Slot::new_from_buffer(managed_buffer!(SLOT))),
        true
    );
}
