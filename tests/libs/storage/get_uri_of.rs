use crate::testing_utils::New;
use crate::{args_set_cid_of, testing_utils};
use customize_nft::libs::storage::StorageModule;
use customize_nft::structs::equippable_attributes_to_render::EquippableAttributesToRender;
use customize_nft::structs::slot::Slot;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm_debug::DebugApi;

use customize_nft::structs::equippable_attributes::EquippableAttributes;
use customize_nft::structs::item::Item;
use elrond_wasm::elrond_codec::multi_types::MultiValue2;
use elrond_wasm_debug::rust_biguint;

#[test]
fn build_url_with_no_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let equippable_attributes = EquippableAttributesToRender {
                attributes: EquippableAttributes::<DebugApi>::new(&[Item::<DebugApi> {
                    name: ManagedBuffer::new_from_bytes(b"item name"),
                    slot: Slot::new_from_bytes(b"hat"),
                }]),
                name: ManagedBuffer::new_from_bytes(b"Equippable #512"),
            };

            let _ = sc.get_uri_of(&equippable_attributes);
        })
        .assert_user_error(
            "There is no URI associated to the attributes hat:item name@Equippable #512.",
        );
}

#[test]
fn build_url_with_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let get_attributes = || EquippableAttributesToRender {
        attributes: EquippableAttributes::<DebugApi>::new(&[Item::<DebugApi> {
            name: ManagedBuffer::new_from_bytes(b"item name"),
            slot: Slot::new_from_bytes(b"hat"),
        }]),
        name: ManagedBuffer::new_from_bytes(b"Equippable #512"),
    };

    setup.enqueue_attributes_to_render(&get_attributes);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let penguin_attributes = get_attributes();

                sc.set_uri_of_attributes(args_set_cid_of!(
                    penguin_attributes.clone(),
                    ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/this is a CID")
                ));

                let url = sc.get_uri_of(&penguin_attributes);

                assert_eq!(
                    url,
                    ManagedBuffer::from(b"https://ipfs.io/ipfs/this is a CID")
                )
            },
        )
        .assert_ok();
}
