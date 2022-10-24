use customize_nft::utils::managed_buffer_utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn return_true_if_only_number() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"1234").is_lowercase(),
        true
    );
}

#[test]
fn return_true_if_lowercase() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"hat").is_lowercase(),
        true
    );
}

#[test]
fn return_true_if_lowercase_with_spaces() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"h at").is_lowercase(),
        true
    );
}

#[test]
fn return_false_if_uppercase() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"HAT").is_lowercase(),
        false
    );
}

#[test]
fn return_false_if_contains_uppercase() {
    DebugApi::dummy();

    assert_eq!(
        ManagedBuffer::<DebugApi>::new_from_bytes(b"hat HAT").is_lowercase(),
        false
    );
}
