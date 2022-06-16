use customize_nft::utils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn to_lowercase() {
    DebugApi::dummy();

    assert_eq!(
        utils::to_lowercase(&ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello World")),
        ManagedBuffer::new_from_bytes(b"hello world")
    );
}
