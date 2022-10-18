use customize_nft::{
    libs::storage::StorageModule, structs::equippable_nft_attributes::EquippableNftAttributes,
};
use elrond_wasm_debug::{rust_biguint, DebugApi};

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
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes = EquippableNftAttributes::<DebugApi>::empty();

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
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes = EquippableNftAttributes::<DebugApi>::empty();

                sc.enqueue_image_to_render(&attributes);
                sc.images_to_render().swap_remove(&attributes);

                assert_eq!(sc.get_images_to_render().len(), 0);
            },
        )
        .assert_ok();
}
