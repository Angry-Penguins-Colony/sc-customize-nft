use core::ops::Deref;

use customize_nft::utils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_split() {
    DebugApi::dummy();

    let input = b"Hello there!";
    let output = utils::split_buffer::<DebugApi>(&ManagedBuffer::new_from_bytes(input), b' ');

    assert_eq!(output.len(), 2);
    assert_eq!(
        output.get(0).deref(),
        &ManagedBuffer::new_from_bytes(b"Hello")
    );
    assert_eq!(
        output.get(1).deref(),
        &ManagedBuffer::new_from_bytes(b"there!")
    );
}

#[test]
fn test_split_while_empty() {
    DebugApi::dummy();

    let input = b"";
    let output = utils::split_buffer::<DebugApi>(&ManagedBuffer::new_from_bytes(input), b' ');

    assert_eq!(output.len(), 0);
}
