use customize_nft::{
    libs::storage::StorageModule, structs::equippable_nft_attributes::EquippableNftAttributes,
};
use elrond_wasm::{elrond_codec::multi_types::MultiValue2, types::MultiValueEncoded};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::{args_set_cid_of, testing_utils};

#[test]
fn works() {
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

                assert_eq!(sc.__images_to_render().len(), 1);
                assert_eq!(sc.__images_to_render().get(1), attributes);
            },
        )
        .assert_ok();
}

#[test]
fn handle_duplicate() {
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
                sc.enqueue_image_to_render(&attributes);

                assert_eq!(sc.__images_to_render().len(), 1);
                assert_eq!(sc.__images_to_render().get(1), attributes);
            },
        )
        .assert_ok();
}

#[test]
fn dont_add_if_in_cid_of() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
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
        .assert_ok();
}
