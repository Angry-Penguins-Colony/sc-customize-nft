use customize_nft::{
    constants::ENQUEUE_PRICE,
    libs::equippable_uris::EquippableUrisModule,
    structs::{equippable_attributes::EquippableAttributes, image_to_render::ImageToRender},
};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::testing_utils;

#[test]
fn returns_empty() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let enqueued = sc.get_images_to_render();
                assert_eq!(enqueued.len(), 0);
            },
        )
        .assert_ok();
}

#[test]
fn returns_one_after_one_enqueue() {
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
                let attributes = ImageToRender {
                    attributes: EquippableAttributes::<DebugApi>::empty(),
                    name: managed_buffer!(b"Equippable #512"),
                };

                sc.enqueue_image_to_render(&attributes);
                assert_eq!(sc.images_to_render().len(), 1);
                assert_eq!(sc.images_to_render().contains(&attributes), true);

                let enqueued = sc.get_images_to_render();
                assert_eq!(enqueued.len(), 1);

                let mut iter = enqueued.into_iter();
                assert_eq!(&iter.next().unwrap(), &attributes);
                assert_eq!(iter.next(), Option::None);
            },
        )
        .assert_ok();
}

#[test]
fn returns_zero_after_one_dequeue() {
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
                let attributes = ImageToRender {
                    attributes: EquippableAttributes::<DebugApi>::empty(),
                    name: managed_buffer!(b"Equippable #512"),
                };

                sc.enqueue_image_to_render(&attributes);
                sc.images_to_render().swap_remove(&attributes);

                assert_eq!(sc.get_images_to_render().len(), 0);
            },
        )
        .assert_ok();
}
