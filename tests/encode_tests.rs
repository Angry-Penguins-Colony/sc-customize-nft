use std::io::Read;

use elrond_wasm::{elrond_codec::TopEncode, types::TokenIdentifier};
use elrond_wasm_debug::DebugApi;
use equip_penguin::structs::{
    item::Item, item_slot::ItemSlot, penguin_attributes::PenguinAttributes,
};

#[test]
fn should_top_encode() {
    DebugApi::dummy();

    let penguin = PenguinAttributes::new(&[(
        &ItemSlot::Hat,
        Item::<DebugApi> {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
            nonce: 1,
        },
    )]);

    let expected = b"{
        \"attributes\":[
                {
                    \"trait_value\":\"Hat\",
                    \"value\":\"HAT-a2b4e5\"
                },
                {
                    \"trait_value\":\"Background\",
                    \"value\":\"unequipped\"
                },
                {
                    \"trait_value\":\"Skin\",
                    \"value\":\"unequipped\"
                },                
                {
                    \"trait_value\":\"Beak\",
                    \"value\":\"unequipped\"
                },
                {
                    \"trait_value\":\"Weapon\",
                    \"value\":\"unequipped\"
                },
                {
                    \"trait_value\":\"Clothes\",
                    \"value\":\"unequipped\"
                },
                {
                    \"trait_value\":\"Eyes\",
                    \"value\":\"unequipped\"
                }
            ]
        }";

    let mut serialized_attributes = Vec::new();
    match penguin.top_encode(&mut serialized_attributes) {
        Ok(_) => {
            // std::str::from_utf8(&err.as_bytes()).unwrap(

            println!(
                "\n========\n{}\n========",
                std::str::from_utf8(&serialized_attributes).unwrap()
            );

            // print!("{}", std::str::from_utf8(&serialized_attributes.).unwrap();

            // assert_eq!(
            //     serialized_attributes, b"hat:HAT-a2b4e5",
            //     "top_encode should return the correct bytes"
            // );
        }
        Err(err) => panic!("top_encode should not fail: {:?}", err),
    }
}
