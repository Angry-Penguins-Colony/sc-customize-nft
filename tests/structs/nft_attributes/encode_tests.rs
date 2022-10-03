use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::elrond_codec::TopEncode;
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn should_top_encode() {
    DebugApi::dummy();

    let equippable_nft_attributes = EquippableNftAttributes::new(&[Item::<DebugApi> {
        name: managed_buffer!(b"Pirate Hat"),
        slot: managed_buffer!(b"hat"),
    }]);

    let expected = b"Hat:Pirate Hat";

    assert_equippable_encode_eq(equippable_nft_attributes, expected);
}

/// no matter the order, the encoding must be sorted alphabetically
#[test]
fn should_top_encode_two() {
    DebugApi::dummy();

    let attributes_order_one = EquippableNftAttributes::new(&[
        Item::<DebugApi> {
            name: managed_buffer!(b"Gun"),
            slot: managed_buffer!(b"weapon"),
        },
        Item::<DebugApi> {
            name: managed_buffer!(b"Pirate Hat"),
            slot: managed_buffer!(b"hat"),
        },
    ]);

    let attributes_order_two = EquippableNftAttributes::new(&[
        Item::<DebugApi> {
            name: managed_buffer!(b"Pirate Hat"),
            slot: managed_buffer!(b"hat"),
        },
        Item::<DebugApi> {
            name: managed_buffer!(b"Gun"),
            slot: managed_buffer!(b"weapon"),
        },
    ]);

    assert_equippable_encode_eq(attributes_order_one, b"Hat:Pirate Hat;Weapon:Gun");
    assert_equippable_encode_eq(attributes_order_two, b"Hat:Pirate Hat;Weapon:Gun");
}

#[test]
fn should_top_encode_after_emptying() {
    DebugApi::dummy();

    let mut equippable_nft_attributes = EquippableNftAttributes::new(&[Item::<DebugApi> {
        name: managed_buffer!(b"Pirate Hat"),
        slot: managed_buffer!(b"hat"),
    }]);
    equippable_nft_attributes.empty_slot(&managed_buffer!(b"hat"));

    let expected = b"Hat:unequipped";

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
