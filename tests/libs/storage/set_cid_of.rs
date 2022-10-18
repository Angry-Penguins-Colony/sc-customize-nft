use customize_nft::{
    libs::storage::{EndpointWrappers, StorageModule},
    structs::equippable_nft_attributes::EquippableNftAttributes,
};
use elrond_wasm::elrond_codec::multi_types::MultiValue2;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::{args_set_cid_of, testing_utils};

#[test]
fn should_set_if_empty() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let cid_bytes = b"some cid";

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes = EquippableNftAttributes::<DebugApi>::empty();
                sc.set_cid_of(args_set_cid_of!(
                    attributes.clone(),
                    managed_buffer!(cid_bytes)
                ));

                assert_eq!(
                    sc.cid_of_attribute(&attributes).get(),
                    managed_buffer!(cid_bytes)
                );
            },
        )
        .assert_ok();
}

#[test]
fn should_set_if_not_emtpy() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let first_cid_bytes = b"some cid";
    let second_cid_bytes = b"another cid";

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes = EquippableNftAttributes::<DebugApi>::empty();

                sc.set_cid_of(args_set_cid_of!(
                    attributes.clone(),
                    managed_buffer!(first_cid_bytes)
                ));
                assert_eq!(
                    sc.cid_of_attribute(&attributes).get(),
                    managed_buffer!(first_cid_bytes)
                );

                sc.set_cid_of(args_set_cid_of!(
                    attributes.clone(),
                    managed_buffer!(second_cid_bytes)
                ));
                assert_eq!(
                    sc.cid_of_attribute(&attributes).get(),
                    managed_buffer!(second_cid_bytes),
                    "first_cid_bytes should be overwrited by second_cid_bytes"
                );
            },
        )
        .assert_ok();
}

#[test]
fn should_fail_if_not_owner() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.first_user_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.call_set_cid_of();
            },
        )
        .assert_user_error("You don't have the permission to call this endpoint.");
}

#[test]
fn should_remove_enqueued_image_to_render() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let first_cid_bytes = b"some cid";

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
                assert_eq!(&sc.images_to_render().get(1), &attributes);

                sc.set_cid_of(args_set_cid_of!(
                    attributes.clone(),
                    managed_buffer!(first_cid_bytes)
                ));

                assert_eq!(
                    sc.images_to_render().len(),
                    0,
                    "The enqueud image to render should be has been removed."
                );
            },
        )
        .assert_ok();
}
