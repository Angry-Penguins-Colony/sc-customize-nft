use customize_nft::utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn return_true_if_exact_equals() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"abc").contains(b"abc"),
        true
    );
}

#[test]
fn return_true_if_contains() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"some_prefixabcsome_suffix").contains(b"abc"),
        true
    );
}

#[test]
fn return_false_because_to_find_is_separated() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"ab_c").contains(b"abc"),
        false
    );
}

#[test]
fn return_false_because_partial_contains() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"ab").contains(b"abcde"),
        false
    );
}
