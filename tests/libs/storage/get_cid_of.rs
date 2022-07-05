use customize_nft::{
    libs::storage::StorageModule,
    structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item},
};
use elrond_wasm::{elrond_codec::multi_types::MultiValue2, types::MultiValueEncoded};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::{args_set_cid_of, testing_utils};

/// The eq of nft_attributes doesn't work on storage. We write these tests to help us fix this.

#[test]
fn should_return_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let cid_bytes = b"some cid";

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let cid_buffer = managed_buffer!(cid_bytes);

                let attributes = EquippableNftAttributes::<DebugApi>::empty();
                sc.set_cid_of(args_set_cid_of!(attributes.clone(), cid_buffer.clone()));

                assert_eq!(sc.get_cid_of(&attributes), cid_buffer)
            },
        )
        .assert_ok();
}

#[test]
fn should_return_cid_from_equivalent_but_not_exact_attr() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let cid_bytes = b"some cid";

    let a_slot = b"hat";
    let a_value = b"Pirate Hat";

    let b_slot = b"badge";
    let b_value = b"1";

    // register a+b
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let cid_buffer = managed_buffer!(cid_bytes);

                let attributes = EquippableNftAttributes::<DebugApi>::new(&[
                    (
                        &managed_buffer!(a_slot),
                        Item::<DebugApi> {
                            name: managed_buffer!(a_value),
                        },
                    ),
                    (
                        &managed_buffer!(b_slot),
                        Item::<DebugApi> {
                            name: managed_buffer!(b_value),
                        },
                    ),
                ]);
                sc.set_cid_of(args_set_cid_of!(attributes.clone(), cid_buffer.clone()));

                assert_eq!(sc.get_cid_of(&attributes), cid_buffer)
            },
        )
        .assert_ok();

    // check if b+a has the same CID
    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let attributes = EquippableNftAttributes::<DebugApi>::new(&[
                (
                    &managed_buffer!(b_slot),
                    Item::<DebugApi> {
                        name: managed_buffer!(b_value),
                    },
                ),
                (
                    &managed_buffer!(a_slot),
                    Item::<DebugApi> {
                        name: managed_buffer!(a_value),
                    },
                ),
            ]);

            assert_eq!(sc.get_cid_of(&attributes), managed_buffer!(cid_bytes));
        })
        .assert_ok();
}
