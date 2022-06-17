use customize_nft::utils::managed_buffer_utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_remove_first_and_last_char() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello").remove_first_and_last_char(),
        ManagedBuffer::new_from_bytes(b"ell")
    );
}
