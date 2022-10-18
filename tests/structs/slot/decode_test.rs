use customize_nft::structs::slot::Slot;
use elrond_wasm::{elrond_codec::TopDecode, types::ManagedBuffer};
use elrond_wasm_debug::DebugApi;

#[test]
fn should_force_lowercase() {
    DebugApi::dummy();

    let input_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(b"HAT");

    let actual_output = Slot::<DebugApi>::top_decode(input_buffer).unwrap();

    assert_eq!(actual_output, Slot::<DebugApi>::new_from_bytes(b"hat"));
}
