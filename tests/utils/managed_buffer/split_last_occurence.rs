use customize_nft::utils::managed_buffer_utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_split_last_occurence() {
    DebugApi::dummy();

    let output =
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello-there-Yes").split_last_occurence(b'-');

    assert_eq!(output.0, ManagedBuffer::new_from_bytes(b"Hello-there"));
    assert_eq!(output.1, ManagedBuffer::new_from_bytes(b"Yes"));
}
