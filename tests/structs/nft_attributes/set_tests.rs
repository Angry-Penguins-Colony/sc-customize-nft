use customize_nft::structs::{item::Item, penguin_attributes::PenguinAttributes};
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::DebugApi;

#[test]
fn set_item_on_empty_slot() {
    DebugApi::dummy();

    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    let mut penguin = PenguinAttributes::<DebugApi>::empty();

    let token = b"ITEM-b";
    let managed_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(token);
    let nonce = 1;

    penguin.set_item(
        &slot,
        Option::Some(Item {
            token: managed_token.clone(),
            nonce: nonce,
            name: ManagedBuffer::new_from_bytes(b"item name"),
        }),
    );

    let item = penguin.get_item(slot).unwrap();

    assert_eq!(item.token, managed_token);
    assert_eq!(item.nonce, nonce);
}

#[test]
#[should_panic]
fn set_item_on_not_empty_slot() {
    DebugApi::dummy();

    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    let mut penguin = PenguinAttributes::<DebugApi>::new(&[(
        slot,
        Item {
            token: TokenIdentifier::from_esdt_bytes(b"ITEM-a"),
            nonce: 0,
            name: ManagedBuffer::new_from_bytes(b"item name"),
        },
    )]);

    let token = b"ITEM-b";
    let managed_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(token);
    let nonce = 1;

    // expect panic
    penguin.set_item(
        &slot,
        Option::Some(Item {
            token: managed_token.clone(),
            nonce: nonce,
            name: ManagedBuffer::new_from_bytes(b"item name"),
        }),
    );
}
