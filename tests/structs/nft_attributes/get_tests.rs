use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::DebugApi;

#[test]
fn get_empty_item() {
    DebugApi::dummy();

    let attributes = EquippableNftAttributes::<DebugApi>::empty();

    let slot = ManagedBuffer::<DebugApi>::new_from_bytes(b"hat");

    assert_eq!(attributes.get_item(&slot).is_none(), true);
}

#[test]
fn get_item() {
    DebugApi::dummy();

    let slot = ManagedBuffer::<DebugApi>::new_from_bytes(b"hat");
    let item = Item {
        name: ManagedBuffer::<DebugApi>::new_from_bytes(b"hat"),
        nonce: 1u64,
        token: TokenIdentifier::<DebugApi>::from_esdt_bytes(b"token"),
    };

    let attributes = EquippableNftAttributes::<DebugApi>::new(&[(&slot, item.clone())]);

    assert_eq!(attributes.get_item(&slot).is_some(), true);
    assert_eq!(attributes.get_item(&slot).unwrap(), item);
}
