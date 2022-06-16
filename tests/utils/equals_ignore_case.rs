use customize_nft::utils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn equals_ignore_case() {
    DebugApi::dummy();

    // equals, even if case is the same
    assert_eq!(
        utils::equals_ignore_case::<DebugApi>(
            &ManagedBuffer::new_from_bytes(b"Hello World"),
            &ManagedBuffer::new_from_bytes(b"Hello World")
        ),
        true
    );

    // case different
    assert_eq!(
        utils::equals_ignore_case::<DebugApi>(
            &ManagedBuffer::new_from_bytes(b"Hello World"),
            &ManagedBuffer::new_from_bytes(b"hello world")
        ),
        true
    );

    // case different
    assert_eq!(
        utils::equals_ignore_case::<DebugApi>(
            &ManagedBuffer::new_from_bytes(b"Hello World"),
            &ManagedBuffer::new_from_bytes(b"See ya World")
        ),
        false
    );

    // same size
    assert_eq!(
        utils::equals_ignore_case::<DebugApi>(
            &ManagedBuffer::new_from_bytes(b"Hello World"),
            &ManagedBuffer::new_from_bytes(b"World Hello")
        ),
        false
    );
}
