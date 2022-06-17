use customize_nft::utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn replace_entire_buffer() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"abc").replace(b"abc", &managed_buffer!(b"123")),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"123")
    );
}

#[test]
fn replace_part() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"someprefix_abc")
            .replace(b"abc", &managed_buffer!(b"123")),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"someprefix_123")
    );
}

#[test]
fn replace_part_twice() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"abcyoyoyoabc")
            .replace(b"abc", &managed_buffer!(b"123")),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"123yoyoyo123")
    );
}

#[test]
fn left_intact_because_no_match() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"abcyoyoyoabc")
            .replace(b"ert", &managed_buffer!(b"123")),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"abcyoyoyoabc")
    );
}

#[test]
fn left_intact_because_partial_match() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"abcyoyoyoabc")
            .replace(b"abcd", &managed_buffer!(b"123")),
        ManagedBuffer::<DebugApi>::new_from_bytes(b"abcyoyoyoabc")
    );
}
