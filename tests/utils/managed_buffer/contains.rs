use customize_nft::utils::managed_buffer_utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn return_true_if_exact_equals() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"a").contains_char(b'a'),
        true
    );
}

#[test]
fn return_true_if_contains() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"some_prefixabcsome_suffix").contains_char(b'a'),
        true
    );
}

#[test]
fn return_false_if_not_found() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"blabla").contains_char(b'z'),
        false
    );
}
