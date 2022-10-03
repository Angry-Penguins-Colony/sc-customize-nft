use customize_nft::structs::item::Item;
use elrond_wasm::elrond_codec::TopEncode;
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn encode_item() {
    DebugApi::dummy();

    let item = Item::<DebugApi> {
        name: managed_buffer!(b"Pirate Hat"),
        slot: managed_buffer!(b"hat"),
    };

    let expected = b"Hat:Pirate Hat";

    assert_item_encode_eq(item, expected);
}

fn assert_item_encode_eq(item: Item<elrond_wasm_debug::tx_mock::TxContextRef>, expected: &[u8]) {
    let mut serialized_attributes = Vec::new();
    match item.top_encode(&mut serialized_attributes) {
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
