use customize_nft::utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn to_lowercase() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello World").to_lowercase(),
        ManagedBuffer::new_from_bytes(b"hello world")
    );
}

#[test]
fn handle_special_characters() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello World@-!").to_lowercase(),
        ManagedBuffer::new_from_bytes(b"hello world@-!")
    );
}
