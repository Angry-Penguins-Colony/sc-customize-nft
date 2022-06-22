use customize_nft::structs::item::Item;
use elrond_wasm::types::{ManagedBuffer};
use elrond_wasm_debug::{managed_buffer, DebugApi};

// todo decode w/ hex as nonce (e.g. "Hat:HAT-a2b4e5-0a")

#[test]
fn decode_item() {
    DebugApi::dummy();

    let input_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(b"Pirate Hat");

    let expected_output = Item::<DebugApi> {
        name: managed_buffer!(b"Pirate Hat"),
    };

    let actual_output = Item::top_decode(&input_buffer).unwrap();

    assert_eq!(expected_output, actual_output);
}
