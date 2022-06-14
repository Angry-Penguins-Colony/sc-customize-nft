use customize_nft::structs::{item::Item, penguin_attributes::PenguinAttributes};
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::DebugApi;

#[test]
fn is_empty_while_not_empty() {
    let slot = &ManagedBuffer::new_from_bytes(b"hat");
    DebugApi::dummy();

    let penguin = PenguinAttributes::<DebugApi>::new(&[(
        slot,
        Item {
            token: TokenIdentifier::from_esdt_bytes(b"ITEM-a"),
            nonce: 0,
            name: ManagedBuffer::new_from_bytes(b"item name"),
        },
    )]);

    assert_eq!(penguin.is_slot_empty(slot), false);
}

#[test]
fn is_empty_while_empty() {
    let slot = &ManagedBuffer::new_from_bytes(b"hat");
    DebugApi::dummy();

    let penguin = PenguinAttributes::<DebugApi>::empty();

    assert_eq!(penguin.is_slot_empty(slot), true);
}
