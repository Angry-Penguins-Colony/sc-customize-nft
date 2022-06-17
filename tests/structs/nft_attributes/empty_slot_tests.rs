use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::DebugApi;

#[test]
fn empty_slot_while_slot_is_empty() {
    DebugApi::dummy();

    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[(
        slot,
        Item {
            token: TokenIdentifier::from_esdt_bytes(b""),
            nonce: 0,
            name: ManagedBuffer::new_from_bytes(b"item name"),
        },
    )]);

    equippable_nft_attributes.empty_slot(&slot);

    assert_eq!(equippable_nft_attributes.is_slot_empty(&slot), true);
}

#[test]
fn empty_slot_while_slot_is_not_empty() {
    DebugApi::dummy();

    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[(
        slot,
        Item {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-aaaa"),
            nonce: 0,
            name: ManagedBuffer::new_from_bytes(b"item name"),
        },
    )]);

    equippable_nft_attributes.empty_slot(&slot);
    assert_eq!(equippable_nft_attributes.is_slot_empty(&slot), true);
}
