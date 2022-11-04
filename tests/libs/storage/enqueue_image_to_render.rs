use customize_nft::{
    constants::{
        ENQUEUE_PRICE, ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_ALREADY_RENDERED,
        ERR_RENDER_ALREADY_IN_QUEUE,
    },
    libs::equippable_uris::EquippableUrisModule,
    structs::{equippable_attributes::EquippableAttributes, item::Item},
};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::testing_utils::{self, New};

#[test]
fn works() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.owner_address, &rust_biguint!(ENQUEUE_PRICE));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(ENQUEUE_PRICE),
            |sc| {
                let attributes = EquippableAttributes::<DebugApi>::empty();
                let name = managed_buffer!(b"Equippable #512");

                sc.enqueue_image_to_render(&attributes, &name);

                assert_eq!(sc.attributes_to_render_by_name().len(), 1);
                assert_eq!(sc.attributes_to_render_by_name().contains_key(&name), true);
            },
        )
        .assert_ok();
}

#[test]
fn enqueue_two_differents_attributes() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.owner_address, &rust_biguint!(ENQUEUE_PRICE));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(ENQUEUE_PRICE),
            |sc| {
                let attributes_a = EquippableAttributes::<DebugApi>::empty();
                let name_a = managed_buffer!(b"Equippable #512");

                let attributes_b = EquippableAttributes::<DebugApi>::new(&[Item {
                    name: managed_buffer!(b"pirate hat"),
                    slot: managed_buffer!(b"hat"),
                }]);
                let name_b = managed_buffer!(b"Equippable #513");

                sc.enqueue_image_to_render(&attributes_a, &name_a);
                sc.enqueue_image_to_render(&attributes_b, &name_b);

                assert_eq!(sc.attributes_to_render_by_name().len(), 2);
                assert_eq!(
                    sc.attributes_to_render_by_name().contains_key(&name_a),
                    true
                );
                assert_eq!(
                    sc.attributes_to_render_by_name().contains_key(&name_b),
                    true
                );

                let mut iter = sc.get_images_to_render().into_iter();
                assert_eq!(iter.next().unwrap().into_tuple(), (attributes_a, name_a));
                assert_eq!(iter.next().unwrap().into_tuple(), (attributes_b, name_b));
                assert_eq!(iter.next().is_none(), true);
            },
        )
        .assert_ok();
}

#[test]
fn panic_if_already_rendererer() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);
    let get_attributes = || {
        (
            EquippableAttributes::<DebugApi>::empty(),
            managed_buffer!(b"Equippable #512"),
        )
    };

    setup.enqueue_and_set_cid_of(&get_attributes, b"https://ipfs.io/ipfs/cid");

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.owner_address, &rust_biguint!(ENQUEUE_PRICE));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(ENQUEUE_PRICE),
            |sc| {
                sc.enqueue_image_to_render(&get_attributes().0, &get_attributes().1);
            },
        )
        .assert_user_error(ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_ALREADY_RENDERED);
}

#[test]
fn panic_if_already_in_queue() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.owner_address, &rust_biguint!(ENQUEUE_PRICE));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(ENQUEUE_PRICE),
            |sc| {
                let attributes = EquippableAttributes::<DebugApi>::empty();
                let name = managed_buffer!(b"Equippable #512");

                sc.enqueue_image_to_render(&attributes, &name);
                sc.enqueue_image_to_render(&attributes, &name);

                assert_eq!(sc.attributes_to_render_by_name().len(), 1);
                assert_eq!(sc.attributes_to_render_by_name().contains_key(&name), true);
            },
        )
        .assert_user_error(ERR_RENDER_ALREADY_IN_QUEUE);
}

#[test]
fn panic_if_attributes_already_in_queue_but_in_another_order() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let image_to_render_a = || {
        (
            EquippableAttributes::<DebugApi>::new(&[
                Item {
                    name: managed_buffer!(b"pirate hat"),
                    slot: managed_buffer!(b"hat"),
                },
                Item {
                    name: managed_buffer!(b"eel"),
                    slot: managed_buffer!(b"beak"),
                },
            ]),
            managed_buffer!(b"Equippable #513"),
        )
    };

    let image_to_render_b = || {
        (
            EquippableAttributes::<DebugApi>::new(&[
                Item {
                    name: managed_buffer!(b"eel"),
                    slot: managed_buffer!(b"beak"),
                },
                Item {
                    name: managed_buffer!(b"pirate hat"),
                    slot: managed_buffer!(b"hat"),
                },
            ]),
            managed_buffer!(b"Equippable #513"),
        )
    };

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.owner_address, &rust_biguint!(ENQUEUE_PRICE * 2));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(ENQUEUE_PRICE),
            |sc| {
                sc.enqueue_image_to_render(&image_to_render_a().0, &image_to_render_a().1);
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(ENQUEUE_PRICE),
            |sc| {
                sc.enqueue_image_to_render(&image_to_render_b().0, &image_to_render_b().1);
            },
        )
        .assert_user_error(ERR_RENDER_ALREADY_IN_QUEUE);
}
