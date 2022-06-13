use customize_nft::constants::ERR_NO_CID_URL;
use customize_nft::libs::penguin_url_builder::PenguinURLBuilder;
use customize_nft::libs::storage::StorageModule;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::SCResult;
use elrond_wasm::types::TokenIdentifier;
use elrond_wasm_debug::DebugApi;
mod testing_utils;

use customize_nft::structs::item::Item;
use customize_nft::structs::penguin_attributes::PenguinAttributes;

#[test]
fn build_url_with_no_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const ITEM_IDENTIFIER: &[u8] = b"ITEM-a1a1a1";
    const NONCE: u64 = 1;

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let penguin_attributes = PenguinAttributes::<DebugApi> {
                hat: Some(Item::<DebugApi> {
                    token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_IDENTIFIER),
                    nonce: NONCE,
                    name: ManagedBuffer::new_from_bytes(b"item name"),
                }),
                ..PenguinAttributes::empty()
            };

            let _ = sc.build_thumbnail_url(&penguin_attributes);
        })
        .assert_user_error(ERR_NO_CID_URL);
}

#[test]
fn build_url_with_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const ITEM_IDENTIFIER: &[u8] = b"ITEM-a1a1a1";
    const NONCE: u64 = 1;

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let penguin_attributes = PenguinAttributes::<DebugApi> {
                hat: Some(Item::<DebugApi> {
                    token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_IDENTIFIER),
                    nonce: NONCE,
                    name: ManagedBuffer::new_from_bytes(b"item name"),
                }),
                ..PenguinAttributes::empty()
            };

            sc.set_thumbnail_cid(
                &penguin_attributes,
                ManagedBuffer::new_from_bytes(b"this is a CID"),
            );

            sc.ipfs_gateway()
                .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/"));

            let url = sc.build_thumbnail_url(&penguin_attributes);

            assert_eq!(
                url,
                SCResult::Ok(ManagedBuffer::from(b"https://ipfs.io/ipfs/this is a CID"))
            )
        })
        .assert_ok();
}

//
// BELOW ARE OLD TESTS, USED AS EXAMPLES
//

// #[test]
// fn build_url_with_no_item() {
//     let mut setup = testing_utils::setup(customize_nft::contract_obj);

//     setup
//         .blockchain_wrapper
//         .execute_query(&setup.cf_wrapper, |sc| {
//             let actual = sc.build_url(
//                 &PenguinAttributes::empty(),
//                 &ManagedBuffer::<DebugApi>::new_from_bytes(b"Penguin #1"),
//             );

//             let mut expected = ManagedBuffer::new();
//             expected.append(&sc.uri().get());
//             expected.append_bytes(b"badge_1/image");

//             assert_eq!(actual, expected);
//         })
//         .assert_ok();
// }

// #[test]
// fn build_url_with_one_item() {
//     // testing_utils::execute_for_all_slot(|mut slot| {
//     let slot = &ItemSlot::Hat;

//     let mut setup = testing_utils::setup(customize_nft::contract_obj);

//     const ITEM_IDENTIFIER: &[u8] = b"ITEM-a1a1a1";
//     const ITEM_TYPE: &[u8] = b"my-item-id";

//     // create item
//     let nonce = setup.register_item(
//         slot.clone(),
//         ITEM_IDENTIFIER,
//         &ItemAttributes {
//             item_id: ManagedBuffer::new_from_bytes(ITEM_TYPE),
//         },
//     );

//     let b_wrapper = &mut setup.blockchain_wrapper;

//     b_wrapper
//         .execute_query(&setup.cf_wrapper, |sc| {
//             // instantiate penguin with item
//             let penguin_attributes = PenguinAttributes::<DebugApi> {
//                 hat: Some(Item::<DebugApi> {
//                     token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_IDENTIFIER),
//                     nonce,
//                     name: ManagedBuffer::new_from_bytes(b"item name"),
//                 }),
//                 ..PenguinAttributes::empty()
//             };

//             let actual = sc.build_url(
//                 &penguin_attributes,
//                 &ManagedBuffer::<DebugApi>::new_from_bytes(b"Penguin #1"),
//             );

//             let mut expected = ManagedBuffer::new();
//             expected.append(&sc.uri().get());
//             expected.append(&ManagedBuffer::new_from_bytes(slot.to_bytes::<DebugApi>())); // slot to string eg. skin
//             expected.append_bytes(b"_");
//             expected.append(&ManagedBuffer::new_from_bytes(ITEM_TYPE)); // slot value eg. albino
//             expected.append_bytes(b"+badge_1/image");

//             assert_eq!(actual, expected);
//         })
//         .assert_ok();
// }

// #[test]
// fn build_url_with_two_item() {
//     // testing_utils::execute_for_all_slot(|mut slot| {

//     let mut setup = testing_utils::setup(customize_nft::contract_obj);

//     const NONCE: u64 = INIT_NONCE;

//     const SLOT_1: &ItemSlot = &ItemSlot::Hat;
//     const ITEM_1_IDENTIFIER: &[u8] = b"ITEM-a1a1a1";
//     const ITEM_1_TYPE: &[u8] = b"first-item";

//     const SLOT_2: &ItemSlot = &ItemSlot::Skin;
//     const ITEM_2_IDENTIFIER: &[u8] = b"ITEM-b2b2b2";
//     const ITEM_2_TYPE: &[u8] = b"second-item";

//     let items: Vec<(&[u8], &[u8], &ItemSlot)> = vec![
//         (ITEM_1_IDENTIFIER, ITEM_1_TYPE, SLOT_1),
//         (ITEM_2_IDENTIFIER, ITEM_2_TYPE, SLOT_2),
//     ];

//     for (id, typeee, slot) in items {
//         let nonce = setup.register_item(
//             slot.clone(),
//             id,
//             &ItemAttributes::<DebugApi> {
//                 item_id: ManagedBuffer::<DebugApi>::new_from_bytes(typeee),
//             },
//         );

//         assert_eq!(nonce, NONCE); // must be the same nonce for all items
//     }

//     let b_wrapper = &mut setup.blockchain_wrapper;

//     b_wrapper
//         .execute_query(&setup.cf_wrapper, |sc| {
//             // instantiate penguin with item
//             let penguin_attributes = PenguinAttributes::<DebugApi>::new(&[
//                 (
//                     &ItemSlot::Hat,
//                     Item::<DebugApi> {
//                         token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_1_IDENTIFIER),
//                         nonce: NONCE,
//                         name: ManagedBuffer::new_from_bytes(b"Pirate hat"),
//                     },
//                 ),
//                 (
//                     &ItemSlot::Skin,
//                     Item::<DebugApi> {
//                         token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_2_IDENTIFIER),
//                         nonce: NONCE,
//                         name: ManagedBuffer::new_from_bytes(b"Albino"),
//                     },
//                 ),
//             ]);

//             let actual = sc.build_url(
//                 &penguin_attributes,
//                 &ManagedBuffer::<DebugApi>::new_from_bytes(b"Penguin #1"),
//             );

//             let mut expected = ManagedBuffer::new();
//             expected.append(&sc.uri().get());
//             expected.append(&ManagedBuffer::new_from_bytes(
//                 SLOT_1.to_bytes::<DebugApi>(),
//             ));
//             expected.append_bytes(b"_");
//             expected.append(&ManagedBuffer::new_from_bytes(ITEM_1_TYPE)); // slot value eg. albino

//             expected.append_bytes(b"+");

//             expected.append(&ManagedBuffer::new_from_bytes(
//                 SLOT_2.to_bytes::<DebugApi>(),
//             ));
//             expected.append_bytes(b"_");
//             expected.append(&ManagedBuffer::new_from_bytes(ITEM_2_TYPE)); // slot value eg. albino

//             expected.append_bytes(b"+badge_1/image");

//             assert_eq!(actual, expected);
//         })
//         .assert_ok();
// }
