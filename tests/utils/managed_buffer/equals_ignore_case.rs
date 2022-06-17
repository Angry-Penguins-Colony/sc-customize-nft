use customize_nft::utils::managed_buffer_utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn equals_ignore_case() {
    DebugApi::dummy();

    // equals, even if case is the same
    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello World")
            .equals_ignore_case(&managed_buffer!(b"Hello World")),
        true
    );

    // case different
    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello World")
            .equals_ignore_case(&managed_buffer!(b"hello world")),
        true
    );

    // case different
    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello World")
            .equals_ignore_case(&managed_buffer!(b"See ya World")),
        false
    );

    // same size
    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello World")
            .equals_ignore_case(&managed_buffer!(b"World Hello")),
        false
    );
}
