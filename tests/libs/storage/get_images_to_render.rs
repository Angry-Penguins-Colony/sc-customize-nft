use customize_nft::{
    constants::ENQUEUE_PRICE, libs::equippable_uris::EquippableUrisModule,
    structs::equippable_attributes::EquippableAttributes,
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
                let attributes = EquippableAttributes::<DebugApi>::empty();
                let name = managed_buffer!(b"Equippable #512");

                sc.enqueue_image_to_render(&attributes, &name);
                assert_eq!(sc.attributes_to_render_by_name().len(), 1);
                assert_eq!(sc.attributes_to_render_by_name().contains_key(&name), true);

                let enqueued = sc.get_images_to_render();
                assert_eq!(enqueued.len(), 1);

                let mut iter = enqueued.into_iter();
                assert_eq!(iter.next().unwrap().into_tuple(), (attributes, name));
                assert_eq!(iter.next().is_none(), true);
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
                let attributes = EquippableAttributes::<DebugApi>::empty();
                let name = managed_buffer!(b"Equippable #512");

                sc.enqueue_image_to_render(&attributes, &name);
                sc.attributes_to_render_by_name().remove(&name);

                assert_eq!(sc.get_images_to_render().len(), 0);
            },
        )
        .assert_ok();
}
