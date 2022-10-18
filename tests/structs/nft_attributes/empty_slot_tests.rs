use customize_nft::structs::{
    equippable_nft_attributes::EquippableNftAttributes, item::Item, slot::Slot,
};
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::testing_utils::{self, New};

#[test]
fn should_empty_slot_if_already_empty() {
    DebugApi::dummy();

    const SLOT: &[u8] = b"hat";

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[]);

    equippable_nft_attributes.empty_slot(&Slot::new_from_buffer(managed_buffer!(SLOT)));

    assert_eq!(
        equippable_nft_attributes.is_slot_empty(&Slot::new_from_buffer(managed_buffer!(SLOT))),
        true
    );
}

#[test]
fn should_empty_slot() {
    DebugApi::dummy();

    const SLOT: &[u8] = b"hat";

    let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[Item {
        name: ManagedBuffer::new_from_bytes(b"item name"),
        slot: Slot::new_from_buffer(managed_buffer!(SLOT)),
    }]);

    equippable_nft_attributes.empty_slot(&Slot::new_from_buffer(managed_buffer!(SLOT)));
    assert_eq!(
        equippable_nft_attributes.is_slot_empty(&Slot::new_from_buffer(managed_buffer!(SLOT))),
        true
    );
}

#[test]
fn should_empty_slot_with_slot_different_case() {
    const SLOT_LOWERCASE: &[u8] = b"hat";
    const SLOT_UPPERCASE: &[u8] = b"HAT";

    let setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.execute_in_managed_environment(|| {
        let mut equippable_nft_attributes = EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(b"item name"),
            slot: Slot::new_from_buffer(managed_buffer!(SLOT_LOWERCASE)),
        }]);

        equippable_nft_attributes
            .empty_slot(&Slot::new_from_buffer(managed_buffer!(SLOT_UPPERCASE)));

        assert_eq!(
            equippable_nft_attributes
                .is_slot_empty(&Slot::new_from_buffer(managed_buffer!(SLOT_LOWERCASE))),
            true
        );
    });
}
