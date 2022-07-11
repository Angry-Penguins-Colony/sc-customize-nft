use customize_nft::{
    constants::ERR_IMAGE_NOT_IN_QUEUE, libs::storage::StorageModule,
    structs::equippable_nft_attributes::EquippableNftAttributes,
};
use elrond_wasm_debug::{rust_biguint, DebugApi};

use crate::testing_utils;

#[test]
fn should_remove_enqueued_image_to_render() {
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
                sc.dequeue_image_to_render(&attributes);

                assert_eq!(sc.__images_to_render().len(), 0);
            },
        )
        .assert_ok();
}

#[test]
fn panic_when_removing_not_in_queue() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes = EquippableNftAttributes::<DebugApi>::empty();

                sc.dequeue_image_to_render(&attributes);

                assert_eq!(sc.__images_to_render().len(), 0);
            },
        )
        .assert_user_error(ERR_IMAGE_NOT_IN_QUEUE);
}
