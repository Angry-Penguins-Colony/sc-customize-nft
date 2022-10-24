use customize_nft::structs::{item::Item, slot::Slot};
use elrond_wasm::{
    elrond_codec::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::{ManagedBuffer, ManagedBufferNestedDecodeInput},
};
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::testing_utils::{self};

#[test]
fn top_should_works() {
    let setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.execute_in_managed_environment(|| {
        let encoded_item = &Item {
            name: managed_buffer!(b"Pirate Hat"),
            slot: Slot::<DebugApi>::new_from_bytes(b"hat"),
        };

        let mut output = ManagedBuffer::<DebugApi>::new_from_bytes(b"");
        let _ = encoded_item.top_encode(&mut output).unwrap();

        print!("top encode {:?}", output);

        let result_decoded_item = Item::<DebugApi>::top_decode(output);

        assert_eq!(
            result_decoded_item.is_ok(),
            true,
            "TopDecode should not fail"
        );
        assert_eq!(encoded_item, &result_decoded_item.unwrap());
    });
}

#[test]
fn nested_should_works() {
    let setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.execute_in_managed_environment(|| {
        let encoded_item = &Item {
            name: managed_buffer!(b"Pirate Hat"),
            slot: Slot::<DebugApi>::new_from_bytes(b"hat"),
        };

        let mut output = ManagedBuffer::<DebugApi>::new_from_bytes(b"");
        let _ = encoded_item.dep_encode(&mut output).unwrap();

        print!("nested encode {:?}", output);

        let mut input = ManagedBufferNestedDecodeInput::new(output);
        let result_decoded_item = Item::<DebugApi>::dep_decode(&mut input);

        assert_eq!(
            result_decoded_item.is_ok(),
            true,
            "NestedDecode should not fail"
        );
        assert_eq!(encoded_item, &result_decoded_item.unwrap());
    });
}
