use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm_debug::{managed_buffer, DebugApi};

use crate::{assert_eq_symetry, assert_ne_symetry};
#[test]
fn both_empty_should_equals() {
    DebugApi::dummy();
    assert_eq_symetry!(
        EquippableNftAttributes::<DebugApi>::empty(),
        EquippableNftAttributes::<DebugApi>::empty()
    );
}

#[test]
fn one_empty_should_not_equals() {
    DebugApi::dummy();

    assert_ne_symetry!(
        EquippableNftAttributes::<DebugApi>::empty(),
        EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: managed_buffer!(b"Pirate Hat"),
            slot: managed_buffer!(b"hat"),
        },])
    );
}

#[test]
fn should_equals_if_same() {
    DebugApi::dummy();

    assert_eq_symetry!(
        EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: managed_buffer!(b"Pirate Hat"),
            slot: managed_buffer!(b"hat"),
        }]),
        EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: managed_buffer!(b"Pirate Hat"),
            slot: managed_buffer!(b"hat"),
        }])
    );
}

#[test]
fn should_equals_if_different_slot_order() {
    DebugApi::dummy();

    assert_eq_symetry!(
        EquippableNftAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: managed_buffer!(b"hat"),
            },
            Item {
                name: managed_buffer!(b"Fishing Rod"),
                slot: managed_buffer!(b"weapon"),
            }
        ]),
        EquippableNftAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Fishing Rod"),
                slot: managed_buffer!(b"weapon"),
            },
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: managed_buffer!(b"hat"),
            }
        ])
    );
}

#[test]
fn different_size_should_return_false() {
    DebugApi::dummy();

    assert_ne_symetry!(
        EquippableNftAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: managed_buffer!(b"hat"),
            },
            Item {
                name: managed_buffer!(b"Fishing Rod"),
                slot: managed_buffer!(b"weapon"),
            },
            Item {
                name: managed_buffer!(b"Background 1"),
                slot: managed_buffer!(b"background"),
            }
        ]),
        EquippableNftAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Fishing Rod"),
                slot: managed_buffer!(b"weapon"),
            },
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: managed_buffer!(b"hat"),
            }
        ])
    );
}

#[test]
fn item_difference_should_false() {
    DebugApi::dummy();

    assert_ne_symetry!(
        EquippableNftAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Katana"),
                slot: managed_buffer!(b"weapon"),
            },
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: managed_buffer!(b"hat"),
            }
        ]),
        EquippableNftAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"Fishing Rod"),
                slot: managed_buffer!(b"weapon"),
            },
            Item {
                name: managed_buffer!(b"Pirate Hat"),
                slot: managed_buffer!(b"hat"),
            }
        ])
    );
}
