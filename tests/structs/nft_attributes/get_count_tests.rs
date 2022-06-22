use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
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
        (
            &ManagedBuffer::new_from_bytes(b"hat"),
            Item {
                name: ManagedBuffer::new_from_bytes(b"pirate hat"),
            },
        ),
        (
            &ManagedBuffer::new_from_bytes(b"background"),
            Item {
                name: ManagedBuffer::new_from_bytes(b"blue bg"),
            },
        ),
    ]);

    assert_eq!(attributes.get_count(), 2);
}

#[test]
fn get_count_expected_one_after_delete() {
    DebugApi::dummy();

    let slot_to_empty = &&ManagedBuffer::new_from_bytes(b"background");
    let mut attributes = EquippableNftAttributes::<DebugApi>::new(&[
        (
            &ManagedBuffer::new_from_bytes(b"hat"),
            Item {
                name: ManagedBuffer::new_from_bytes(b"pirate hat"),
            },
        ),
        (
            slot_to_empty,
            Item {
                name: ManagedBuffer::new_from_bytes(b"blue bg"),
            },
        ),
    ]);

    attributes.empty_slot(slot_to_empty);

    assert_eq!(attributes.get_count(), 1);
}
