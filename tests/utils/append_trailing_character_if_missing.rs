use customize_nft::utils;
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn should_append() {
    DebugApi::dummy();

    assert_eq!(
        utils::append_trailing_character_if_missing::<DebugApi>(
            &managed_buffer!(b"Hello World"),
            b'!'
        ),
        managed_buffer!(b"Hello World!")
    );
}

#[test]
fn should_not_append() {
    DebugApi::dummy();

    assert_eq!(
        utils::append_trailing_character_if_missing::<DebugApi>(
            &managed_buffer!(b"Hello World!"),
            b'!'
        ),
        managed_buffer!(b"Hello World!")
    );
}
