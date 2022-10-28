use customize_nft::{
    constants::{
        ENQUEUE_PRICE, ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_ALREADY_RENDERED,
        ERR_RENDER_ALREADY_IN_QUEUE,
    },
    libs::storage::StorageModule,
    structs::{
        equippable_attributes::EquippableAttributes,
        equippable_attributes_to_render::EquippableAttributesToRender, item::Item, slot::Slot,
    },
    Equip,
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
                let attributes = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::empty(),
                    name: managed_buffer!(b"Equippable #512"),
                };

                sc.enqueue_image_to_render(&attributes);

                assert_eq!(sc.images_to_render().len(), 1);
                assert_eq!(sc.images_to_render().contains(&attributes), true);
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
                let attributes_a = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::empty(),
                    name: managed_buffer!(b"Equippable #512"),
                };
                let attributes_b = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::new(&[Item {
                        name: managed_buffer!(b"pirate hat"),
                        slot: Slot::new_from_bytes(b"hat"),
                    }]),
                    name: managed_buffer!(b"Equippable #513"),
                };

                sc.enqueue_image_to_render(&attributes_a);
                sc.enqueue_image_to_render(&attributes_b);

                assert_eq!(sc.images_to_render().len(), 2);
                assert_eq!(sc.images_to_render().contains(&attributes_a), true);
                assert_eq!(sc.images_to_render().contains(&attributes_b), true);

                let mut iter = sc.get_images_to_render().into_iter();
                assert_eq!(iter.next(), Some(attributes_a));
                assert_eq!(iter.next(), Some(attributes_b));
                assert_eq!(iter.next(), None);
            },
        )
        .assert_ok();
}

#[test]
fn panic_if_already_rendererer() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);
    let get_attributes = || EquippableAttributesToRender {
        attributes: EquippableAttributes::<DebugApi>::empty(),
        name: managed_buffer!(b"Equippable #512"),
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
                sc.enqueue_image_to_render(&get_attributes());
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
                let attributes = EquippableAttributesToRender {
                    attributes: EquippableAttributes::<DebugApi>::empty(),
                    name: managed_buffer!(b"Equippable #512"),
                };

                sc.enqueue_image_to_render(&attributes);
                sc.enqueue_image_to_render(&attributes);

                assert_eq!(sc.images_to_render().len(), 1);
                assert_eq!(sc.images_to_render().contains(&attributes), true);
            },
        )
        .assert_user_error(ERR_RENDER_ALREADY_IN_QUEUE);
}

#[test]
fn panic_if_attributes_already_in_queue_but_in_another_order() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let attributes_a = || EquippableAttributesToRender {
        attributes: EquippableAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"pirate hat"),
                slot: Slot::new_from_bytes(b"hat"),
            },
            Item {
                name: managed_buffer!(b"eel"),
                slot: Slot::new_from_bytes(b"beak"),
            },
        ]),
        name: managed_buffer!(b"Equippable #513"),
    };

    let attributes_b = || EquippableAttributesToRender {
        attributes: EquippableAttributes::<DebugApi>::new(&[
            Item {
                name: managed_buffer!(b"eel"),
                slot: Slot::new_from_bytes(b"beak"),
            },
            Item {
                name: managed_buffer!(b"pirate hat"),
                slot: Slot::new_from_bytes(b"hat"),
            },
        ]),
        name: managed_buffer!(b"Equippable #513"),
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
                sc.enqueue_image_to_render(&attributes_a());
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
                sc.enqueue_image_to_render(&attributes_b());
            },
        )
        .assert_user_error(ERR_RENDER_ALREADY_IN_QUEUE);
}
