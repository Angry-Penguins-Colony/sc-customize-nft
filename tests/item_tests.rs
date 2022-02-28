use elrond_wasm::types::TokenIdentifier;
use elrond_wasm_debug::DebugApi;
use equip_penguin::structs::item::Item;

#[test]
fn test_new() {
    DebugApi::dummy();

    let input_str = b"HAT-a2b4e5-01";
    let expected_output = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 1,
    };

    let actual_output = Item::<DebugApi>::new(input_str);

    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_decode_with_nonce_10() {
    DebugApi::dummy();

    let input_str = b"HAT-a2b4e5-0a";
    let expected_output = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 10,
    };

    let actual_output = Item::<DebugApi>::new(input_str);

    assert_eq!(expected_output, actual_output);
}
