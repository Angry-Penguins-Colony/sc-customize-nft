use customize_nft::structs::{
    equippable_nft_attributes::EquippableNftAttributes, item::Item, slot::Slot,
};
use elrond_wasm::elrond_codec::TopEncode;
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::testing_utils;

#[test]
fn should_top_encode() {
    DebugApi::dummy();

    let equippable_nft_attributes = EquippableNftAttributes::new(&[Item::<DebugApi> {
        name: managed_buffer!(b"Pirate Hat"),
        slot: Slot::new_from_bytes(b"hat"),
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
            slot: Slot::new_from_bytes(b"weapon"),
        },
        Item::<DebugApi> {
            name: managed_buffer!(b"Pirate Hat"),
            slot: Slot::new_from_bytes(b"hat"),
        },
    ]);

    let attributes_order_two = EquippableNftAttributes::new(&[
        Item::<DebugApi> {
            name: managed_buffer!(b"Pirate Hat"),
            slot: Slot::new_from_bytes(b"hat"),
        },
        Item::<DebugApi> {
            name: managed_buffer!(b"Gun"),
            slot: Slot::new_from_bytes(b"weapon"),
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
        slot: Slot::new_from_bytes(b"hat"),
    }]);
    equippable_nft_attributes.empty_slot(&Slot::new_from_bytes(b"hat"));

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

#[test]
fn panic_if_more_that_512_bytes() {
    const NAME: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer fermentum posuere lorem nec posuere. Phasellus ex sapien, aliquet sed lacus nec, tincidunt blandit mi. Fusce pellentesque, libero vel varius elementum, enim magna accumsan nisi, congue auctor sem augue finibus tellus. Fusce ultrices sapien quis orci finibus rutrum et pretium augue. Donec faucibus semper molestie. Curabitur id eros a odio consequat vestibulum volutpat et sapien. Nullam aliquet augue ac nibh scelerisque, in faucibus purus tincidunt. Ut ac congue enim. Donec imperdiet luctus est. Sed sollicitudin ipsum ut velit interdum hendrerit. Sed ullamcorper, lorem eu dapibus vulputate, libero massa tempor erat, eget interdum diam augue sed mi. Donec egestas, nisi at ullamcorper lobortis, dolor odio commodo elit, in facilisis odio arcu ut lectus. Pellentesque fermentum elit nunc, vel tempor mi aliquam sed. Nam iaculis finibus consequat.";

    assert_eq!(
        NAME.len() > 512,
        true,
        "The input will not test the hit of the load bytes"
    );

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_| {
            let equippable_nft_attributes = EquippableNftAttributes::new(&[Item::<DebugApi> {
                slot: Slot::new_from_bytes(b"lorem"),
                name: managed_buffer!(NAME),
            }]);

            let mut serialized_attributes = Vec::new();
            let _ = equippable_nft_attributes.top_encode(&mut serialized_attributes);
        })
        .assert_user_error("ManagedBuffer is too big");
}
