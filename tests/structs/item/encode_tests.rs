use customize_nft::structs::item::Item;
use elrond_wasm::{elrond_codec::TopEncode, types::TokenIdentifier};
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn encode_item_with_nonce_1() {
    DebugApi::dummy();

    let item = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 1,
        name: managed_buffer!(b"Pirate Hat"),
    };

    let expected = b"Pirate Hat (HAT-a2b4e5-01)";

    assert_item_encode_eq(item, expected);
}

#[test]
fn encode_item_with_nonce_10() {
    DebugApi::dummy();

    let item = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 10,
        name: managed_buffer!(b"Pirate Hat"),
    };

    let expected = b"Pirate Hat (HAT-a2b4e5-0a)";

    assert_item_encode_eq(item, expected);
}

fn assert_item_encode_eq(item: Item<elrond_wasm_debug::tx_mock::TxContextRef>, expected: &[u8]) {
    let mut serialized_attributes = Vec::new();
    match item.top_encode(&mut serialized_attributes) {
        Ok(_) => {
            println!(
                "\n========\nActual:\n{}\n\nExpected:\n{}\n========\n",
                std::str::from_utf8(&serialized_attributes).unwrap(),
                std::str::from_utf8(expected).unwrap()
            );

            assert_eq!(
                serialized_attributes, expected,
                "top_encode should return the correct bytes"
            );
        }
        Err(err) => panic!("top_encode should not fail: {:?}", err),
    }
}
