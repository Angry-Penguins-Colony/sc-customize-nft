use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::{
    elrond_codec::TopEncode,
    types::{ManagedBuffer, TokenIdentifier},
};
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn should_top_encode() {
    DebugApi::dummy();

    let penguin = EquippableNftAttributes::new(&[(
        &ManagedBuffer::new_from_bytes(b"hat"),
        Item::<DebugApi> {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
            nonce: 1,
            name: managed_buffer!(b"Pirate Hat"),
        },
    )]);

    let expected = b"Hat:Pirate Hat (HAT-a2b4e5-01)";

    assert_penguin_encode_eq(penguin, expected);
}

#[test]
fn should_top_encode_with_nonce_equals_0a() {
    DebugApi::dummy();

    let penguin = EquippableNftAttributes::new(&[(
        &ManagedBuffer::new_from_bytes(b"hat"),
        Item::<DebugApi> {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
            nonce: 10,
            name: managed_buffer!(b"Pirate Hat"),
        },
    )]);

    let expected = b"Hat:Pirate Hat (HAT-a2b4e5-0a)";

    assert_penguin_encode_eq(penguin, expected);
}

fn assert_penguin_encode_eq(
    penguin: EquippableNftAttributes<elrond_wasm_debug::tx_mock::TxContextRef>,
    expected: &[u8],
) {
    let mut serialized_attributes = Vec::new();
    match penguin.top_encode(&mut serialized_attributes) {
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
