use customize_nft::structs::{item::Item, penguin_attributes::PenguinAttributes};
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::DebugApi;

#[test]
fn empty_slot_while_slot_is_empty() {
    let slot = &ManagedBuffer::new_from_bytes(b"hat");
    DebugApi::dummy();

    let mut penguin = PenguinAttributes::<DebugApi>::new(&[(
        slot,
        Item {
            token: TokenIdentifier::from_esdt_bytes(b""),
            nonce: 0,
            name: ManagedBuffer::new_from_bytes(b"item name"),
        },
    )]);

    let result = penguin.empty_slot(&slot);
    assert_eq!(result, Result::Ok(()));
}

#[test]
fn empty_slot_while_slot_is_not_empty() {
    let slot = &ManagedBuffer::new_from_bytes(b"hat");
    DebugApi::dummy();

    let mut penguin = PenguinAttributes::<DebugApi>::new(&[(
        slot,
        Item {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-aaaa"),
            nonce: 0,
            name: ManagedBuffer::new_from_bytes(b"item name"),
        },
    )]);

    let result = penguin.empty_slot(&slot);
    assert_eq!(result, Result::Ok(()));
}
