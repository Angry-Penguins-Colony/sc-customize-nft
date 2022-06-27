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

/// no matter the order, the encoding must be sorted alphabetically
#[test]
fn should_top_encode_two() {
    DebugApi::dummy();

    let attributes_order_one = EquippableNftAttributes::new(&[
        (
            &ManagedBuffer::new_from_bytes(b"weapon"),
            Item::<DebugApi> {
                name: managed_buffer!(b"Gun"),
            },
        ),
        (
            &ManagedBuffer::new_from_bytes(b"hat"),
            Item::<DebugApi> {
                name: managed_buffer!(b"Pirate Hat"),
            },
        ),
    ]);

    let attributes_order_two = EquippableNftAttributes::new(&[
        (
            &ManagedBuffer::new_from_bytes(b"hat"),
            Item::<DebugApi> {
                name: managed_buffer!(b"Pirate Hat"),
            },
        ),
        (
            &ManagedBuffer::new_from_bytes(b"weapon"),
            Item::<DebugApi> {
                name: managed_buffer!(b"Gun"),
            },
        ),
    ]);

    assert_equippable_encode_eq(attributes_order_one, b"Hat:Pirate Hat;Weapon:Gun");
    assert_equippable_encode_eq(attributes_order_two, b"Hat:Pirate Hat;Weapon:Gun");
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
