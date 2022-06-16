use customize_nft::utils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_split_last_occurence() {
    DebugApi::dummy();

    let input = &ManagedBuffer::new_from_bytes(b"Hello-there-Yes");
    let output = utils::split_last_occurence::<DebugApi>(input, b'-');

    assert_eq!(output.0, ManagedBuffer::new_from_bytes(b"Hello-there"));
    assert_eq!(output.1, ManagedBuffer::new_from_bytes(b"Yes"));
}
