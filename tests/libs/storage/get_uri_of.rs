use crate::testing_utils::New;
use crate::{args_set_cid_of, testing_utils};
use customize_nft::libs::storage::StorageModule;
use customize_nft::structs::slot::Slot;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm_debug::DebugApi;

use customize_nft::structs::equippable_nft_attributes::EquippableNftAttributes;
use customize_nft::structs::item::Item;
use elrond_wasm::elrond_codec::multi_types::MultiValue2;
use elrond_wasm_debug::rust_biguint;

#[test]
fn build_url_with_no_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let equippable_attributes =
                EquippableNftAttributes::<DebugApi>::new(&[Item::<DebugApi> {
                    name: ManagedBuffer::new_from_bytes(b"item name"),
                    slot: Slot::new_from_buffer(ManagedBuffer::new_from_bytes(b"hat")),
                }]);

            let _ = sc.get_uri_of(&equippable_attributes);
        })
        .assert_user_error("There is no CID associated to the attributes Hat:item name.");
}

#[test]
fn build_url_with_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let penguin_attributes =
                    EquippableNftAttributes::<DebugApi>::new(&[Item::<DebugApi> {
                        name: ManagedBuffer::new_from_bytes(b"item name"),
                        slot: Slot::new_from_buffer(ManagedBuffer::new_from_bytes(b"hat")),
                    }]);

                sc.set_cid_of(args_set_cid_of!(
                    penguin_attributes.clone(),
                    ManagedBuffer::new_from_bytes(b"this is a CID")
                ));

                sc.ipfs_gateway()
                    .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/"));

                let url = sc.get_uri_of(&penguin_attributes);

                assert_eq!(
                    url,
                    ManagedBuffer::from(b"https://ipfs.io/ipfs/this is a CID")
                )
            },
        )
        .assert_ok();
}
