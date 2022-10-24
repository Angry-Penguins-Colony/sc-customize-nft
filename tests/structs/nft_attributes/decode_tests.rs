use customize_nft::structs::{
    equippable_nft_attributes::EquippableNftAttributes, item::Item, slot::Slot,
};
use elrond_wasm::{elrond_codec::TopDecode, types::ManagedBuffer};
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::testing_utils::New;

#[test]
fn decode_equippable_nft() {
    DebugApi::dummy();

    let input_data = b"Hat:Pirate Hat";
    let input_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(input_data);

    let expected_output = EquippableNftAttributes::new(&[Item::<DebugApi> {
        name: managed_buffer!(b"Pirate Hat"),
        slot: Slot::new_from_buffer(managed_buffer!(b"hat")),
    }]);

    let actual_output = EquippableNftAttributes::top_decode(input_buffer).unwrap();

    assert_eq!(expected_output, actual_output);
}

#[test]
fn decode_empty_equippable_nft() {
    DebugApi::dummy();

    let attributes_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(b"");
    let actual_output = EquippableNftAttributes::<DebugApi>::top_decode(attributes_buffer).unwrap();

    assert_eq!(EquippableNftAttributes::empty(), actual_output);
}

#[test]
fn should_equals() {
    DebugApi::dummy();

    let a_bytes = ManagedBuffer::<DebugApi>::new_from_bytes(b"Hat:Pirate Hat;Badge:1");
    let b_bytes = ManagedBuffer::<DebugApi>::new_from_bytes(b"Badge:1;Hat:Pirate Hat");

    let a = EquippableNftAttributes::<DebugApi>::top_decode(a_bytes).unwrap();
    let b = EquippableNftAttributes::<DebugApi>::top_decode(b_bytes).unwrap();

    assert!(a == b);
}

#[test]
fn should_not_equals() {
    DebugApi::dummy();

    let a_bytes = ManagedBuffer::<DebugApi>::new_from_bytes(b"hat:Pirate Hat;badge:1");
    let b_bytes = ManagedBuffer::<DebugApi>::new_from_bytes(b"badge:10;hat:Pirate Hat");

    let a = EquippableNftAttributes::<DebugApi>::top_decode(a_bytes).unwrap();
    let b = EquippableNftAttributes::<DebugApi>::top_decode(b_bytes).unwrap();

    assert!(a != b);
}
