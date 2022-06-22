use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::{elrond_codec::TopEncode, types::ManagedBuffer};
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn should_top_encode() {
    DebugApi::dummy();

    let equippable_nft_attributes = EquippableNftAttributes::new(&[(
        &ManagedBuffer::new_from_bytes(b"hat"),
        Item::<DebugApi> {
            name: managed_buffer!(b"Pirate Hat"),
        },
    )]);

    let expected = b"Hat:Pirate Hat";

    assert_equippable_encode_eq(equippable_nft_attributes, expected);
}

#[test]
fn should_top_encode_with_nonce_equals_0a() {
    DebugApi::dummy();

    let equippable_nft_attributes = EquippableNftAttributes::new(&[(
        &ManagedBuffer::new_from_bytes(b"hat"),
        Item::<DebugApi> {
            name: managed_buffer!(b"Pirate Hat"),
        },
    )]);

    let expected = b"Hat:Pirate Hat";

    assert_equippable_encode_eq(equippable_nft_attributes, expected);
}

fn assert_equippable_encode_eq(
    equippable_nft_attributes: EquippableNftAttributes<elrond_wasm_debug::tx_mock::TxContextRef>,
    expected: &[u8],
) {
    let mut serialized_attributes = Vec::new();
    match equippable_nft_attributes.top_encode(&mut serialized_attributes) {
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
