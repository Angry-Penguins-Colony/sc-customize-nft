use customize_nft::utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_to_u64() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"0a").hex_to_u64(),
        Some(10)
    );
    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"00").hex_to_u64(),
        Some(0)
    );
    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"01").hex_to_u64(),
        Some(1)
    );
    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"lol").hex_to_u64(),
        None
    );
    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"-1").hex_to_u64(),
        None
    );
}
