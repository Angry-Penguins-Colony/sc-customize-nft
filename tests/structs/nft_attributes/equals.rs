use customize_nft::structs::{equippable_attributes::EquippableAttributes, item::Item, slot::Slot};
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::{assert_eq_symetry, assert_ne_symetry, testing_utils::New};
#[test]
fn both_empty_should_equals() {
    DebugApi::dummy();
    assert_eq_symetry!(
        EquippableAttributes::<DebugApi>::empty(),
        EquippableAttributes::<DebugApi>::empty()
    );
}

#[test]
fn one_empty_should_not_equals() {
    DebugApi::dummy();

    assert_ne_symetry!(
        EquippableAttributes::<DebugApi>::empty(),
        EquippableAttributes::<DebugApi>::new(&[Item {
            name: managed_buffer!(b"Pirate Hat"),
            slot: Slot::new_from_bytes(b"hat"),
        },])
    );
}

#[test]
fn should_equals_if_same() {
    DebugApi::dummy();

    assert_eq_symetry!(
        EquippableAttributes::<DebugApi>::new(&[Item {
            name: managed_buffer!(b"Pirate Hat"),
            slot: Slot::new_from_bytes(b"hat"),
        }]),
        EquippableAttributes::<DebugApi>::new(&[Item {
            name: managed_buffer!(b"Pirate Hat"),
            slot: Slot::new_from_bytes(b"hat"),
        }])
    );
}

#[test]
fn should_equals_if_different_slot_order() {
    DebugApi::dummy();

    assert_eq_symetry!(
        EquippableAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: Slot::new_from_bytes(b"hat"),
            },
            Item {
                name: managed_buffer!(b"Fishing Rod"),
                slot: Slot::new_from_bytes(b"weapon"),
            }
        ]),
        EquippableAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Fishing Rod"),
                slot: Slot::new_from_bytes(b"weapon"),
            },
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: Slot::new_from_bytes(b"hat"),
            }
        ])
    );
}

#[test]
fn different_size_should_return_false() {
    DebugApi::dummy();

    assert_ne_symetry!(
        EquippableAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: Slot::new_from_bytes(b"hat"),
            },
            Item {
                name: managed_buffer!(b"Fishing Rod"),
                slot: Slot::new_from_bytes(b"weapon"),
            },
            Item {
                name: managed_buffer!(b"Background 1"),
                slot: Slot::new_from_bytes(b"background"),
            }
        ]),
        EquippableAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Fishing Rod"),
                slot: Slot::new_from_bytes(b"weapon"),
            },
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: Slot::new_from_bytes(b"hat"),
            }
        ])
    );
}

#[test]
fn item_difference_should_false() {
    DebugApi::dummy();

    assert_ne_symetry!(
        EquippableAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Katana"),
                slot: Slot::new_from_bytes(b"weapon"),
            },
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: Slot::new_from_bytes(b"hat"),
            }
        ]),
        EquippableAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Fishing Rod"),
                slot: Slot::new_from_bytes(b"weapon"),
            },
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: Slot::new_from_bytes(b"hat"),
            }
        ])
    );
}
