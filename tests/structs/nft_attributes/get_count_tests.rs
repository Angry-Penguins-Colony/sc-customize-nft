use customize_nft::structs::{
    equippable_nft_attributes::EquippableNftAttributes, item::Item, slot::Slot,
};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn get_count_while_empty() {
    DebugApi::dummy();

    let attributes = EquippableNftAttributes::<DebugApi>::empty();

    assert_eq!(attributes.get_count(), 0);
}

#[test]
fn get_count_expected_two() {
    DebugApi::dummy();

    let attributes = EquippableNftAttributes::<DebugApi>::new(&[
        Item {
            name: ManagedBuffer::new_from_bytes(b"pirate hat"),
            slot: Slot::new_from_bytes(b"hat"),
        },
        Item {
            name: ManagedBuffer::new_from_bytes(b"blue bg"),
            slot: Slot::new_from_bytes(b"background"),
        },
    ]);

    assert_eq!(attributes.get_count(), 2);
}

#[test]
fn get_count_expected_one_after_delete() {
    DebugApi::dummy();

    const SLOT: &[u8] = b"background";

    let mut attributes = EquippableNftAttributes::<DebugApi>::new(&[
        Item {
            name: ManagedBuffer::new_from_bytes(b"pirate hat"),
            slot: Slot::new_from_bytes(b"hat"),
        },
        Item {
            name: ManagedBuffer::new_from_bytes(b"blue bg"),
            slot: Slot::new_from_bytes(SLOT),
        },
    ]);

    attributes.empty_slot(&Slot::new_from_bytes(SLOT));

    assert_eq!(attributes.get_count(), 1);
}
