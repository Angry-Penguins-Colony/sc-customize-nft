use customize_nft::structs::{equippable_attributes::EquippableAttributes, item::Item};
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::testing_utils::New;

#[test]
fn should_empty_slot_if_already_empty() {
    DebugApi::dummy();

    const SLOT: &[u8] = b"hat";

    let mut equippable_nft_attributes = EquippableAttributes::<DebugApi>::new(&[]);

    equippable_nft_attributes.empty_slot(&managed_buffer!(SLOT));

    assert_eq!(
        equippable_nft_attributes.is_slot_empty(&managed_buffer!(SLOT)),
        true
    );
}

#[test]
fn should_empty_slot() {
    DebugApi::dummy();

    const SLOT: &[u8] = b"hat";

    let mut equippable_nft_attributes = EquippableAttributes::<DebugApi>::new(&[Item {
        name: managed_buffer!(b"item name"),
        slot: managed_buffer!(SLOT),
    }]);

    equippable_nft_attributes.empty_slot(&managed_buffer!(SLOT));
    assert_eq!(
        equippable_nft_attributes.is_slot_empty(&managed_buffer!(SLOT)),
        true
    );
}
