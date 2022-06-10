use elrond_wasm::{
    elrond_codec::TopDecode,
    types::{ManagedBuffer, TokenIdentifier},
};
use elrond_wasm_debug::{managed_buffer, DebugApi};
use customize_nft::structs::{
    item::Item, item_slot::ItemSlot, penguin_attributes::PenguinAttributes,
};

#[test]
fn decode_penguin() {
    DebugApi::dummy();

    let input_data=b"Hat:Pirate Hat (HAT-a2b4e5-01);Background:unequipped;Skin:unequipped;Beak:unequipped;Weapon:unequipped;Clothes:unequipped;Eyes:unequipped";
    let input_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(input_data);

    let expected_output = PenguinAttributes::new(&[(
        &ItemSlot::Hat,
        Item::<DebugApi> {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
            nonce: 1,
            name: managed_buffer!(b"Pirate Hat"),
        },
    )]);

    let actual_output = PenguinAttributes::top_decode(input_buffer).unwrap();

    assert_eq!(expected_output, actual_output);
}

// todo decode w/ hex as nonce (e.g. "Hat:HAT-a2b4e5-0a")

#[test]
fn decode_item() {
    DebugApi::dummy();

    let input_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(b"Pirate Hat (HAT-a2b4e5-01)");

    let expected_output = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 1,
        name: managed_buffer!(b"Pirate Hat"),
    };

    let actual_output = Item::top_decode(&input_buffer).unwrap();

    assert_eq!(expected_output, actual_output);
}

#[test]
fn decode_slot() {
    DebugApi::dummy();

    assert_eq!(
        ItemSlot::from(ManagedBuffer::<DebugApi>::new_from_bytes(b"Hat")),
        ItemSlot::Hat
    );
    assert_eq!(
        ItemSlot::from(ManagedBuffer::<DebugApi>::new_from_bytes(b"Beak")),
        ItemSlot::Beak
    );
    assert_eq!(
        ItemSlot::from(ManagedBuffer::<DebugApi>::new_from_bytes(b"Clothes")),
        ItemSlot::Clothes
    );
    assert_eq!(
        ItemSlot::from(ManagedBuffer::<DebugApi>::new_from_bytes(b"Eyes")),
        ItemSlot::Eye
    );
    assert_eq!(
        ItemSlot::from(ManagedBuffer::<DebugApi>::new_from_bytes(b"Weapon")),
        ItemSlot::Weapon
    );
    assert_eq!(
        ItemSlot::from(ManagedBuffer::<DebugApi>::new_from_bytes(b"Skin")),
        ItemSlot::Skin
    );
    assert_eq!(
        ItemSlot::from(ManagedBuffer::<DebugApi>::new_from_bytes(b"Background")),
        ItemSlot::Background
    );
    assert_eq!(
        ItemSlot::from(ManagedBuffer::<DebugApi>::new_from_bytes(b"azeazzeaea")),
        ItemSlot::None
    );
}
