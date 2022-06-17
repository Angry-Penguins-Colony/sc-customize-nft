use customize_nft::utils::u64_utils::UtilsU64;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_u64_to_ascii() {
    DebugApi::dummy();

    assert_eq!(
        10.to_ascii::<DebugApi>(),
        ManagedBuffer::new_from_bytes(b"10")
    );

    assert_eq!(
        1.to_ascii::<DebugApi>(),
        ManagedBuffer::new_from_bytes(b"1")
    );

    assert_eq!(
        0.to_ascii::<DebugApi>(),
        ManagedBuffer::new_from_bytes(b"0")
    );
}
