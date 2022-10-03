use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn is_empty_while_not_empty() {
    DebugApi::dummy();
    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    let equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(b"item name"),
        slot: slot.clone(),
    }]);

    assert_eq!(equippable_nft_attributes.is_slot_empty(slot), false);
}

#[test]
fn is_empty_while_empty() {
    DebugApi::dummy();
    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    let equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::empty();

    assert_eq!(equippable_nft_attributes.is_slot_empty(slot), true);
}
