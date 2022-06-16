use customize_nft::utils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_remove_first_char() {
    DebugApi::dummy();

    assert_eq!(
        utils::remove_first_char::<DebugApi>(&ManagedBuffer::new_from_bytes(b"Hello")),
        ManagedBuffer::new_from_bytes(b"ello")
    );
}
