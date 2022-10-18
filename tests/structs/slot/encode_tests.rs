use customize_nft::structs::slot::Slot;
use elrond_wasm::elrond_codec::TopEncode;
use elrond_wasm_debug::DebugApi;

#[test]
fn should_force_lowercase() {
    DebugApi::dummy();

    let slot = Slot::<DebugApi>::new_from_bytes(b"HAT");

    let expected = b"hat";

    assert_item_encode_eq(slot, expected);
}

fn assert_item_encode_eq(slot: Slot<elrond_wasm_debug::tx_mock::TxContextRef>, expected: &[u8]) {
    let mut serialized_attributes = Vec::new();
    match slot.top_encode(&mut serialized_attributes) {
        Ok(_) => {
            println!(
                "\n========\nActual:\n{}\n\nExpected:\n{}\n========\n",
                std::str::from_utf8(&serialized_attributes).unwrap(),
                std::str::from_utf8(expected).unwrap()
            );

            assert_eq!(
                serialized_attributes, expected,
                "top_encode should return the correct bytes"
            );
        }
        Err(err) => panic!("top_encode should not fail: {:?}", err),
    }
}
