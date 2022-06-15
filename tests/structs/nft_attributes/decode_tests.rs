use customize_nft::structs::{item::Item, penguin_attributes::PenguinAttributes};
use elrond_wasm::{
    elrond_codec::TopDecode,
    types::{ManagedBuffer, TokenIdentifier},
};
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn decode_penguin() {
    DebugApi::dummy();

    let input_data = b"Hat:Pirate Hat (HAT-a2b4e5-01)";
    let input_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(input_data);

    let expected_output = PenguinAttributes::new(&[(
        &ManagedBuffer::new_from_bytes(b"Hat"),
        Item::<DebugApi> {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
            nonce: 1,
            name: managed_buffer!(b"Pirate Hat"),
        },
    )]);

    let actual_output = PenguinAttributes::top_decode(input_buffer).unwrap();

    assert_eq!(expected_output, actual_output);
}

#[test]
fn decode_penguin_empty() {
    DebugApi::dummy();

    let attributes_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(b"");
    let actual_output = PenguinAttributes::<DebugApi>::top_decode(attributes_buffer).unwrap();

    assert_eq!(PenguinAttributes::empty(), actual_output);
}
