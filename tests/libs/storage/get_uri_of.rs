use crate::testing_utils::New;
use crate::{args_set_cid_of, testing_utils};
use customize_nft::libs::equippable_uris::EquippableUrisModule;
use customize_nft::structs::slot::Slot;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm_debug::DebugApi;

use customize_nft::structs::equippable_attributes::EquippableAttributes;
use customize_nft::structs::item::Item;
use elrond_wasm::elrond_codec::multi_types::MultiValue3;
use elrond_wasm_debug::rust_biguint;

#[test]
fn build_url_with_no_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let attributes = EquippableAttributes::<DebugApi>::new(&[Item::<DebugApi> {
                name: ManagedBuffer::new_from_bytes(b"item name"),
                slot: Slot::new_from_bytes(b"hat"),
            }]);
            let name = ManagedBuffer::new_from_bytes(b"Equippable #512");

            let _ = sc.get_uri_of(&attributes, &name);
        })
        .assert_user_error(
            "There is no URI associated to the attributes hat:item name for Equippable #512.",
        );
}

#[test]
fn build_url_with_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let get_image_to_render = || {
        (
            EquippableAttributes::<DebugApi>::new(&[Item::<DebugApi> {
                name: ManagedBuffer::new_from_bytes(b"item name"),
                slot: Slot::new_from_bytes(b"hat"),
            }]),
            ManagedBuffer::new_from_bytes(b"Equippable #512"),
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
                    ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/this is a CID")
                ));

                let url = sc.get_uri_of(&image_to_render.0, &image_to_render.1);

                assert_eq!(
                    url,
                    ManagedBuffer::from(b"https://ipfs.io/ipfs/this is a CID")
                )
            },
        )
        .assert_ok();
}
