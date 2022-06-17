use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
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
                token: TokenIdentifier::from_esdt_bytes(b"HAT-aaaa"),
                nonce: 0,
                name: ManagedBuffer::new_from_bytes(b"pirate hat"),
            },
        ),
        (
            &ManagedBuffer::new_from_bytes(b"background"),
            Item {
                token: TokenIdentifier::from_esdt_bytes(b"BG-aaaa"),
                nonce: 0,
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
                token: TokenIdentifier::from_esdt_bytes(b"HAT-aaaa"),
                nonce: 0,
                name: ManagedBuffer::new_from_bytes(b"pirate hat"),
            },
        ),
        (
            slot_to_empty,
            Item {
                token: TokenIdentifier::from_esdt_bytes(b"BG-aaaa"),
                nonce: 0,
                name: ManagedBuffer::new_from_bytes(b"blue bg"),
            },
        ),
    ]);

    attributes.empty_slot(slot_to_empty);

    assert_eq!(attributes.get_count(), 1);
}
