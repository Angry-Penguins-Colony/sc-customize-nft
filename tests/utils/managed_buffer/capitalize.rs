use customize_nft::utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn capitalize() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"hello world").capitalize(),
        ManagedBuffer::new_from_bytes(b"Hello world")
    );
}

#[test]
fn capitalize_already_capitalized() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello world").capitalize(),
        ManagedBuffer::new_from_bytes(b"Hello world")
    );
}
