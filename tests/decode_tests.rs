use elrond_wasm::{
    elrond_codec::TopDecode,
    types::{ManagedBuffer, TokenIdentifier},
};
use elrond_wasm_debug::DebugApi;
use equip_penguin::structs::{
    item::Item, item_slot::ItemSlot, penguin_attributes::PenguinAttributes,
};

#[test]
fn decode_test() {
    DebugApi::dummy();

    let input_data = b"{\"attributes\":[{\"trait_type\":\"hat\",\"value\":\"HAT-a2b4e5\"},{\"trait_type\":\"background\",\"value\":\"unequipped\"},{\"trait_type\":\"skin\",\"value\":\"unequipped\"},{\"trait_type\":\"beak\",\"value\":\"unequipped\"},{\"trait_type\":\"weapon\",\"value\":\"unequipped\"},{\"trait_type\":\"clothes\",\"value\":\"unequipped\"},{\"trait_type\":\"eyes\",\"value\":\"unequipped\"}]}";
    let input_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(input_data);

    let expected_output = PenguinAttributes::new(&[(
        &ItemSlot::Hat,
        Item::<DebugApi> {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
            nonce: 1,
        },
    )]);

    let actual_output = PenguinAttributes::top_decode(input_buffer).unwrap();

    assert_eq!(expected_output, actual_output);
}
