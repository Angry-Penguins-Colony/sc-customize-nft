use customize_nft::utils::u64_utils::UtilsU64;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_some_random_u64() {
    DebugApi::dummy();

    assert_eq!(
        10.to_hex(),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"0a")
    );

    assert_eq!(1.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"01"));
    assert_eq!(0.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"00"));
}

#[test]
fn test_each_digit() {
    DebugApi::dummy();

    assert_eq!(0.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"00"));
    assert_eq!(1.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"01"));
    assert_eq!(2.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"02"));
    assert_eq!(3.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"03"));
    assert_eq!(4.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"04"));
    assert_eq!(5.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"05"));
    assert_eq!(6.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"06"));
    assert_eq!(7.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"07"));
    assert_eq!(8.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"08"));
    assert_eq!(9.to_hex(), ManagedBuffer::<DebugApi>::new_from_bytes(b"09"));
    assert_eq!(
        10.to_hex(),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"0a")
    );
    assert_eq!(
        11.to_hex(),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"0b")
    );
    assert_eq!(
        12.to_hex(),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"0c")
    );
    assert_eq!(
        13.to_hex(),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"0d")
    );
    assert_eq!(
        14.to_hex(),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"0e")
    );
    assert_eq!(
        15.to_hex(),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"0f")
    );
}
