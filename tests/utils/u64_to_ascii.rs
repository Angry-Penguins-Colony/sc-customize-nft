use customize_nft::utils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_u64_to_ascii() {
    DebugApi::dummy();

    assert_eq!(
        utils::u64_to_ascii::<DebugApi>(&10),
        ManagedBuffer::new_from_bytes(b"10")
    );

    assert_eq!(
        utils::u64_to_ascii::<DebugApi>(&1),
        ManagedBuffer::new_from_bytes(b"1")
    );

    assert_eq!(
        utils::u64_to_ascii::<DebugApi>(&0),
        ManagedBuffer::new_from_bytes(b"0")
    );
}
