use customize_nft::utils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_to_u64() {
    DebugApi::dummy();

    assert_eq!(
        utils::hex_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"0a")),
        Some(10)
    );
    assert_eq!(
        utils::hex_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"00")),
        Some(0)
    );
    assert_eq!(
        utils::hex_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"01")),
        Some(1)
    );
    assert_eq!(
        utils::hex_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"lol")),
        None
    );
    assert_eq!(
        utils::hex_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"-1")),
        None
    );
}
