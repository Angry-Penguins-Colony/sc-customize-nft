use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn set_item_on_empty_slot() {
    DebugApi::dummy();

    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::empty();

    equippable_nft_attributes.set_item(
        &slot,
        Option::Some(Item {
            name: ManagedBuffer::new_from_bytes(b"item name"),
        }),
    );

    let item = equippable_nft_attributes.get_item(slot).unwrap();

    assert_eq!(item.name, b"item name");
}

#[test]
#[should_panic]
fn set_item_on_not_empty_slot() {
    DebugApi::dummy();

    let slot = &ManagedBuffer::new_from_bytes(b"hat");

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[(
        slot,
        Item {
            name: ManagedBuffer::new_from_bytes(b"item name"),
        },
    )]);

    // expect panic
    equippable_nft_attributes.set_item(
        &slot,
        Option::Some(Item {
            name: ManagedBuffer::new_from_bytes(b"item name"),
        }),
    );
}
