use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn empty_slot_while_slot_is_empty() {
    DebugApi::dummy();

    const SLOT: &[u8] = b"hat";

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(b"item name"),
        slot: managed_buffer!(SLOT),
    }]);

    equippable_nft_attributes.empty_slot(&managed_buffer!(SLOT));

    assert_eq!(
        equippable_nft_attributes.is_slot_empty(&managed_buffer!(SLOT)),
        true
    );
}

#[test]
fn empty_slot_while_slot_is_not_empty() {
    DebugApi::dummy();

    const SLOT: &[u8] = b"hat";

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(b"item name"),
        slot: managed_buffer!(SLOT),
    }]);

    equippable_nft_attributes.empty_slot(&managed_buffer!(SLOT));
    assert_eq!(
        equippable_nft_attributes.is_slot_empty(&managed_buffer!(SLOT)),
        true
    );
}
