use std::ops::Deref;

use customize_nft::utils::managed_buffer_utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn test_split() {
    DebugApi::dummy();

    let input = b"Hello there!";
    let output = &ManagedBuffer::<DebugApi>::new_from_bytes(input).split(b' ');

    assert_eq!(output.len(), 2);
    assert_eq!(output.get(0).deref(), &managed_buffer!(b"Hello"));
    assert_eq!(output.get(1).deref(), &managed_buffer!(b"there!"));
}

#[test]
fn test_split_while_empty() {
    DebugApi::dummy();

    let input = b"";
    let output = ManagedBuffer::<DebugApi>::new_from_bytes(input).split(b' ');

    assert_eq!(output.len(), 0);
}
