use customize_nft::utils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn capitalize() {
    DebugApi::dummy();

    assert_eq!(
        utils::capitalize::<DebugApi>(&ManagedBuffer::new_from_bytes(b"hello world")),
        ManagedBuffer::new_from_bytes(b"Hello world")
    );
}

#[test]
fn capitalize_already_capitalized() {
    DebugApi::dummy();

    assert_eq!(
        utils::capitalize::<DebugApi>(&ManagedBuffer::new_from_bytes(b"Hello world")),
        ManagedBuffer::new_from_bytes(b"Hello world")
    );
}
