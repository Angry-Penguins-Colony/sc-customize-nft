use customize_nft::utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn should_append() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello World")
            .append_trailing_character_if_missing(b'!'),
        managed_buffer!(b"Hello World!")
    );
}

#[test]
fn should_not_append() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"Hello World@")
            .append_trailing_character_if_missing(b'@'),
        managed_buffer!(b"Hello World@")
    );
}
