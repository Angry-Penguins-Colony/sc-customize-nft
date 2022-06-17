use crate::testing_utils;
use customize_nft::constants::ERR_NO_CID_URL;
use customize_nft::libs::penguin_url_builder::PenguinURLBuilder;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::TokenIdentifier;
use elrond_wasm_debug::DebugApi;

use customize_nft::structs::equippable_nft_attributes::EquippableNftAttributes;
use customize_nft::structs::item::Item;

#[test]
fn build_url_with_no_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const ITEM_IDENTIFIER: &[u8] = b"ITEM-a1a1a1";
    const NONCE: u64 = 1;

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let penguin_attributes = EquippableNftAttributes::<DebugApi>::new(&[(
                &ManagedBuffer::new_from_bytes(b"hat"),
                Item::<DebugApi> {
                    token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_IDENTIFIER),
                    nonce: NONCE,
                    name: ManagedBuffer::new_from_bytes(b"item name"),
                },
            )]);

            let _ = sc.build_thumbnail_url(&penguin_attributes);
        })
        .assert_user_error(ERR_NO_CID_URL);
}

// #[test]
// fn build_url_with_associated_cid() {
//     let mut setup = testing_utils::setup(customize_nft::contract_obj);

//     const ITEM_IDENTIFIER: &[u8] = b"ITEM-a1a1a1";
//     const NONCE: u64 = 1;

//     setup
//         .blockchain_wrapper
//         .execute_query(&setup.cf_wrapper, |sc| {
//             let penguin_attributes = PenguinAttributes::<DebugApi>::new(&[(
//                 &ManagedBuffer::new_from_bytes(b"hat"),
//                 Item::<DebugApi> {
//                     token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_IDENTIFIER),
//                     nonce: NONCE,
//                     name: ManagedBuffer::new_from_bytes(b"item name"),
//                 },
//             )]);

//             sc.set_thumbnail_cid(
//                 &penguin_attributes,
//                 ManagedBuffer::new_from_bytes(b"this is a CID"),
//             );

//             sc.ipfs_gateway()
//                 .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/"));

//             let url = sc.build_thumbnail_url(&penguin_attributes);

//             assert_eq!(
//                 url,
//                 SCResult::Ok(ManagedBuffer::from(b"https://ipfs.io/ipfs/this is a CID"))
//             )
//         })
//         .assert_ok();
// }
