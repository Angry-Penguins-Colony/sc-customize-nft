use customize_nft::{
    constants::{
        ENQUEUE_PRICE, ERR_CANNOT_OVERRIDE_URI_OF_ATTRIBUTE, ERR_IMAGE_NOT_IN_RENDER_QUEUE,
    },
    libs::equippable_uris::{EndpointWrappers, EquippableUrisModule},
    structs::equippable_attributes::EquippableAttributes,
};
use elrond_wasm::elrond_codec::multi_types::MultiValue3;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::{args_set_cid_of, testing_utils};

#[test]
fn should_set_if_empty() {
    DebugApi::dummy();

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let cid_bytes = b"https://ipfs.io/ipfs/some cid";

    let get_image_to_render = || {
        (
            EquippableAttributes::<DebugApi>::empty(),
            managed_buffer!(b"Equippable #512"),
        )
    };
    setup.enqueue_attributes_to_render(&get_image_to_render);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_uri_of_attributes(args_set_cid_of!(
                    get_image_to_render().0,
                    get_image_to_render().1,
                    managed_buffer!(cid_bytes)
                ));

                assert_eq!(
                    sc.uris_of_attributes(&get_image_to_render().0, &get_image_to_render().1)
                        .get(),
                    managed_buffer!(cid_bytes)
                );
            },
        )
        .assert_ok();
}

#[test]
fn panic_if_not_in_render_queue() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let cid_bytes = b"https://ipfs.io/ipfs/some cid";

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes = EquippableAttributes::<DebugApi>::empty();
                let name = managed_buffer!(b"Equippable #512");

                sc.set_uri_of_attributes(args_set_cid_of!(
                    attributes,
                    name,
                    managed_buffer!(cid_bytes)
                ));

                assert_eq!(
                    sc.uris_of_attributes(&attributes, &name).get(),
                    managed_buffer!(cid_bytes)
                );
            },
        )
        .assert_user_error(ERR_IMAGE_NOT_IN_RENDER_QUEUE);
}

#[test]
fn panic_if_override_previously_set_uri() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let first_cid_bytes = b"https://ipfs.io/ipfs/some cid";
    let second_cid_bytes = b"https://ipfs.io/ipfs/another cid";

    let get_image_to_render = || {
        (
            EquippableAttributes::<DebugApi>::empty(),
            managed_buffer!(b"Equippable #512"),
        )
    };

    setup.enqueue_attributes_to_render(&get_image_to_render);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let image_to_render = get_image_to_render();

                sc.set_uri_of_attributes(args_set_cid_of!(
                    image_to_render.0,
                    image_to_render.1,
                    managed_buffer!(first_cid_bytes)
                ));
                assert_eq!(
                    sc.uris_of_attributes(&image_to_render.0, &image_to_render.1)
                        .get(),
                    managed_buffer!(first_cid_bytes)
                );
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let image_to_render = get_image_to_render();

                sc.set_uri_of_attributes(args_set_cid_of!(
                    image_to_render.0,
                    image_to_render.1,
                    managed_buffer!(second_cid_bytes)
                ));
            },
        )
        .assert_user_error(ERR_CANNOT_OVERRIDE_URI_OF_ATTRIBUTE);
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
                sc.call_set_uri_of_attributes();
            },
        )
        .assert_user_error("You don't have the permission to call this endpoint.");
}

#[test]
fn should_remove_enqueued_image_to_render() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let first_cid_bytes = b"https://ipfs.io/ipfs/some cid";

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

                sc.set_uri_of_attributes(args_set_cid_of!(
                    attributes,
                    name,
                    managed_buffer!(first_cid_bytes)
                ));

                assert_eq!(
                    sc.attributes_to_render_by_name().len(),
                    0,
                    "The enqueud image to render should be has been removed."
                );
            },
        )
        .assert_ok();
}
