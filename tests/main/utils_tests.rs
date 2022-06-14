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
fn test_split_last_occurence() {
    DebugApi::dummy();

    let input = &ManagedBuffer::new_from_bytes(b"Hello-there-Yes");
    let output = utils::split_last_occurence::<DebugApi>(input, b'-');

    assert_eq!(output.0, ManagedBuffer::new_from_bytes(b"Hello-there"));
    assert_eq!(output.1, ManagedBuffer::new_from_bytes(b"Yes"));
}

#[test]
fn test_remove_first_char() {
    DebugApi::dummy();

    assert_eq!(
        utils::remove_first_char::<DebugApi>(&ManagedBuffer::new_from_bytes(b"Hello")),
        ManagedBuffer::new_from_bytes(b"ello")
    );
}

#[test]
fn test_remove_first_and_last_char() {
    DebugApi::dummy();

    assert_eq!(
        utils::remove_first_and_last_char::<DebugApi>(&ManagedBuffer::new_from_bytes(b"Hello")),
        ManagedBuffer::new_from_bytes(b"ell")
    );
}

#[test]
fn test_to_u64() {
    DebugApi::dummy();

    assert_eq!(
        utils::hex_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"0a")),
        Some(10)
    );
    assert_eq!(
        utils::hex_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"00")),
        Some(0)
    );
    assert_eq!(
        utils::hex_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"01")),
        Some(1)
    );
    assert_eq!(
        utils::hex_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"lol")),
        None
    );
    assert_eq!(
        utils::hex_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"-1")),
        None
    );
}

#[test]
fn test_ascii_to_u64() {
    DebugApi::dummy();

    assert_eq!(
        utils::ascii_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"010")),
        Some(10)
    );
    assert_eq!(
        utils::ascii_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"00")),
        Some(0)
    );
    assert_eq!(
        utils::ascii_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"01")),
        Some(1)
    );
    assert_eq!(
        utils::ascii_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"lol")),
        None
    );
    assert_eq!(
        utils::ascii_to_u64::<DebugApi>(&ManagedBuffer::new_from_bytes(b"-1")),
        None
    );
}

#[test]
fn test_u64_to_hex() {
    DebugApi::dummy();

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&10),
        ManagedBuffer::new_from_bytes(b"0a")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&1),
        ManagedBuffer::new_from_bytes(b"01")
    );

    assert_eq!(
        utils::u64_to_hex::<DebugApi>(&0),
        ManagedBuffer::new_from_bytes(b"00")
    );
}

#[test]
fn test_u64_to_ascii() {
    DebugApi::dummy();

    assert_eq!(
        utils::u64_to_ascii::<DebugApi>(&10),
        ManagedBuffer::new_from_bytes(b"10")
    );

    assert_eq!(
        utils::u64_to_ascii::<DebugApi>(&1),
        ManagedBuffer::new_from_bytes(b"1")
    );

    assert_eq!(
        utils::u64_to_ascii::<DebugApi>(&0),
        ManagedBuffer::new_from_bytes(b"0")
    );
}

#[test]
fn test_get_number_from_penguin_name() {
    DebugApi::dummy();

    assert_eq!(
        utils::get_number_from_penguin_name(&ManagedBuffer::<DebugApi>::new_from_bytes(
            b"Penguin #1"
        )),
        Some(1)
    );

    assert_eq!(
        utils::get_number_from_penguin_name(&ManagedBuffer::<DebugApi>::new_from_bytes(
            b"Penguin #15"
        )),
        Some(15)
    );
}
