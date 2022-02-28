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

    let expected=b"Hat:HAT-a2b4e5;Background:unequipped;Skin:unequipped;Beak:unequipped;Weapon:unequipped;Clothes:unequipped;Eyes:unequipped";

    let mut serialized_attributes = Vec::new();
    match penguin.top_encode(&mut serialized_attributes) {
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
