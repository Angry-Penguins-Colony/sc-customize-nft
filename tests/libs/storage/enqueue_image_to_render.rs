use customize_nft::{
    constants::{
        ENQUEUE_PRICE, ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_CID_ALREADY_RENDERER,
        ERR_RENDER_ALREADY_IN_QUEUE,
    },
    libs::storage::StorageModule,
    structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item},
};
use elrond_wasm::{elrond_codec::multi_types::MultiValue2, types::MultiValueEncoded};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::{args_set_cid_of, testing_utils};

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
                let attributes = EquippableNftAttributes::<DebugApi>::empty();

                sc.enqueue_image_to_render(&attributes);

                assert_eq!(sc.__images_to_render().len(), 1);
                assert_eq!(sc.__images_to_render().get(1), attributes);
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
                let attributes_a = EquippableNftAttributes::<DebugApi>::empty();
                let attributes_b = EquippableNftAttributes::<DebugApi>::new(&[Item {
                    name: managed_buffer!(b"pirate hat"),
                    slot: managed_buffer!(b"hat"),
                }]);

                sc.enqueue_image_to_render(&attributes_a);
                sc.enqueue_image_to_render(&attributes_b);

                assert_eq!(sc.__images_to_render().len(), 2);
                assert_eq!(sc.__images_to_render().get(1), attributes_a);
                assert_eq!(sc.__images_to_render().get(2), attributes_b);

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
                assert_eq!(sc.__images_to_render().len(), 0);
                {
                    let attributes = EquippableNftAttributes::<DebugApi>::empty();

                    sc.set_cid_of(args_set_cid_of!(
                        attributes.clone(),
                        managed_buffer!(b"cid")
                    ));

                    sc.enqueue_image_to_render(&attributes);
                }
                assert_eq!(sc.__images_to_render().len(), 0);
            },
        )
        .assert_user_error(ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_CID_ALREADY_RENDERER);
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
                let attributes = EquippableNftAttributes::<DebugApi>::empty();

                sc.enqueue_image_to_render(&attributes);
                sc.enqueue_image_to_render(&attributes);

                assert_eq!(sc.__images_to_render().len(), 1);
                assert_eq!(sc.__images_to_render().get(1), attributes);
            },
        )
        .assert_user_error(ERR_RENDER_ALREADY_IN_QUEUE);
}
