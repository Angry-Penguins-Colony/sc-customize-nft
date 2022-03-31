use elrond_wasm::{
    elrond_codec::TopEncode,
    types::{ManagedBuffer, TokenIdentifier},
};
use elrond_wasm_debug::{managed_buffer, DebugApi};
use equip_penguin::structs::item::Item;

#[test]
fn test_new() {
    DebugApi::dummy();

    let expected = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 1,
        name: ManagedBuffer::<DebugApi>::new_from_bytes(b"item name"),
    };

    let input_str = b"item name (HAT-a2b4e5-01)";
    let input_buffer: ManagedBuffer<DebugApi> = managed_buffer!(input_str);
    let actual = Item::<DebugApi>::top_decode(&input_buffer).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_decode_with_nonce_10() {
    DebugApi::dummy();

    let expected = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 10,
        name: ManagedBuffer::<DebugApi>::new_from_bytes(b"item name"),
    };

    let input_str = b"item name (HAT-a2b4e5-0a)";
    let input_buffer: ManagedBuffer<DebugApi> = managed_buffer!(input_str);
    let actual = Item::<DebugApi>::top_decode(&input_buffer).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_encode() {
    DebugApi::dummy();

    let input = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 1,
        name: ManagedBuffer::<DebugApi>::new_from_bytes(b"Hat"),
    };

    let expected: ManagedBuffer<DebugApi> = managed_buffer!(b"Hat (HAT-a2b4e5-01)");

    let mut actual = ManagedBuffer::new();
    let result = input.top_encode(&mut actual);

    assert_eq!(result.is_ok(), true);
    assert_eq!(expected, actual);
}

#[test]
fn test_encode_with_nonce_10() {
    DebugApi::dummy();

    let input = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 10,
        name: ManagedBuffer::<DebugApi>::new_from_bytes(b"Hat"),
    };

    let expected: ManagedBuffer<DebugApi> = managed_buffer!(b"Hat (HAT-a2b4e5-0a)");

    let mut actual = ManagedBuffer::new();
    let result = input.top_encode(&mut actual);

    assert_eq!(result.is_ok(), true);
    assert_eq!(expected, actual);
}
