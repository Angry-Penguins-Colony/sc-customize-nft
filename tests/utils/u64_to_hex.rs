use customize_nft::utils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_some_random_u64() {
    DebugApi::dummy();

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&10),
        ManagedBuffer::new_from_bytes(b"0a")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&1),
        ManagedBuffer::new_from_bytes(b"01")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&0),
        ManagedBuffer::new_from_bytes(b"00")
    );
}

#[test]
fn test_each_digit() {
    DebugApi::dummy();

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&0),
        ManagedBuffer::new_from_bytes(b"00")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&1),
        ManagedBuffer::new_from_bytes(b"01")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&2),
        ManagedBuffer::new_from_bytes(b"02")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&3),
        ManagedBuffer::new_from_bytes(b"03")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&4),
        ManagedBuffer::new_from_bytes(b"04")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&5),
        ManagedBuffer::new_from_bytes(b"05")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&6),
        ManagedBuffer::new_from_bytes(b"06")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&7),
        ManagedBuffer::new_from_bytes(b"07")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&8),
        ManagedBuffer::new_from_bytes(b"08")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&9),
        ManagedBuffer::new_from_bytes(b"09")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&10),
        ManagedBuffer::new_from_bytes(b"0a")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&11),
        ManagedBuffer::new_from_bytes(b"0b")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&12),
        ManagedBuffer::new_from_bytes(b"0c")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&13),
        ManagedBuffer::new_from_bytes(b"0d")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&14),
        ManagedBuffer::new_from_bytes(b"0e")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&15),
        ManagedBuffer::new_from_bytes(b"0f")
    );
}
