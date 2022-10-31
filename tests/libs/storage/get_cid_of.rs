use customize_nft::{
    libs::equippable_uris::EquippableUrisModule,
    structs::{equippable_attributes::EquippableAttributes, item::Item, slot::Slot},
};
use elrond_wasm::{elrond_codec::multi_types::MultiValue3, types::MultiValueEncoded};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::{
    args_set_cid_of,
    testing_utils::{self, New},
};

/// The eq of nft_attributes doesn't work on storage. We write these tests to help us fix this.

#[test]
fn should_return_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let cid_bytes = b"https://ipfs.io/ipfs/some cid";

    let get_attributes = || {
        (
            EquippableAttributes::<DebugApi>::empty(),
            managed_buffer!(b"Equippable #512"),
        )
    };
    setup.enqueue_attributes_to_render(&get_attributes);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let cid_buffer = managed_buffer!(cid_bytes);
                sc.set_uri_of_attributes(args_set_cid_of!(
                    get_attributes().0,
                    get_attributes().1,
                    cid_buffer.clone()
                ));

                assert_eq!(
                    sc.get_uri_of(&get_attributes().0, &get_attributes().1),
                    cid_buffer
                )
            },
        )
        .assert_ok();
}

#[test]
fn should_return_cid_from_equivalent_but_not_exact_attr() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let cid_bytes = b"https://ipfs.io/ipfs/some cid";

    let a_slot = b"hat";
    let a_value = b"Pirate Hat";

    let b_slot = b"badge";
    let b_value = b"1";

    let get_attributes = || {
        (
            EquippableAttributes::<DebugApi>::new(&[
                Item::<DebugApi> {
                    name: managed_buffer!(a_value),
                    slot: Slot::new_from_bytes(a_slot),
                },
                Item::<DebugApi> {
                    name: managed_buffer!(b_value),
                    slot: Slot::new_from_bytes(b_slot),
                },
            ]),
            managed_buffer!(b"Equippable #512"),
        )
    };

    let get_attributes_reversed = || {
        (
            EquippableAttributes::<DebugApi>::new(&[
                Item::<DebugApi> {
                    name: managed_buffer!(b_value),
                    slot: Slot::new_from_bytes(b_slot),
                },
                Item::<DebugApi> {
                    name: managed_buffer!(a_value),
                    slot: Slot::new_from_bytes(a_slot),
                },
            ]),
            managed_buffer!(b"Equippable #512"),
        )
    };

    setup.enqueue_attributes_to_render(&get_attributes);

    // register a+b
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let cid_buffer = managed_buffer!(cid_bytes);

                let image_to_render = get_attributes();
                sc.set_uri_of_attributes(args_set_cid_of!(
                    image_to_render.0,
                    image_to_render.1,
                    cid_buffer.clone()
                ));

                assert_eq!(
                    sc.get_uri_of(&image_to_render.0, &image_to_render.1),
                    cid_buffer
                )
            },
        )
        .assert_ok();

    // check if b+a has the same CID
    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            assert_eq!(
                sc.get_uri_of(&get_attributes_reversed().0, &get_attributes_reversed().1),
                managed_buffer!(cid_bytes)
            );
        })
        .assert_ok();
}
