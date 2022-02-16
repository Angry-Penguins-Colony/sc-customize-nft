use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::TokenIdentifier;
use elrond_wasm_debug::DebugApi;
use equip_penguin::item_slot::ItemSlot;
use equip_penguin::Equip;
use equip_penguin::{
    item::Item, item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes,
};

mod utils;

use utils::INIT_NONCE;

#[test]
fn build_url_with_no_item() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let actual = sc.build_url(&PenguinAttributes::empty());

            assert!(actual.is_ok());

            let mut expected = ManagedBuffer::new();
            expected.append(&sc.uri().get());
            expected.append_bytes(b"empty/image.png");

            assert_eq!(actual.unwrap(), expected);
        })
        .assert_ok();
}

#[test]
fn build_url_with_one_item() {
    // utils::execute_for_all_slot(|mut slot| {
    let slot = &ItemSlot::Hat;

    let mut setup = utils::setup(equip_penguin::contract_obj);

    const ITEM_IDENTIFIER: &[u8] = b"ITEM-a1a1a1";
    const ITEM_TYPE: &[u8] = b"my-item-id";

    // create item
    let nonce = setup.register_item(
        slot.clone(),
        ITEM_IDENTIFIER,
        &ItemAttributes {
            item_id: ManagedBuffer::new_from_bytes(ITEM_TYPE),
        },
    );

    let b_wrapper = &mut setup.blockchain_wrapper;

    let _ = b_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            // instantiate penguin with item
            let penguin_attributes = PenguinAttributes::<DebugApi> {
                hat: Some(Item::<DebugApi> {
                    token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_IDENTIFIER),
                    nonce,
                }),
                ..PenguinAttributes::empty()
            };

            let actual = sc.build_url(&penguin_attributes);

            assert!(actual.is_ok());

            let mut expected = ManagedBuffer::new();
            expected.append(&sc.uri().get());
            expected.append(&ManagedBuffer::new_from_bytes(slot.to_bytes::<DebugApi>())); // slot to string eg. skin
            expected.append_bytes(b"_");
            expected.append(&ManagedBuffer::new_from_bytes(ITEM_TYPE)); // slot value eg. albino
            expected.append_bytes(b"/image.png");

            assert_eq!(actual.unwrap(), expected);
        })
        .assert_ok();
}

#[test]
fn build_url_with_two_item() {
    // utils::execute_for_all_slot(|mut slot| {

    let mut setup = utils::setup(equip_penguin::contract_obj);

    const NONCE: u64 = INIT_NONCE;

    const SLOT_1: &ItemSlot = &ItemSlot::Hat;
    const ITEM_1_IDENTIFIER: &[u8] = b"ITEM-a1a1a1";
    const ITEM_1_TYPE: &[u8] = b"first-item";

    const SLOT_2: &ItemSlot = &ItemSlot::Skin;
    const ITEM_2_IDENTIFIER: &[u8] = b"ITEM-b2b2b2";
    const ITEM_2_TYPE: &[u8] = b"second-item";

    let items: Vec<(&[u8], &[u8], &ItemSlot)> = vec![
        (ITEM_1_IDENTIFIER, ITEM_1_TYPE, SLOT_1),
        (ITEM_2_IDENTIFIER, ITEM_2_TYPE, SLOT_2),
    ];

    for (id, typeee, slot) in items {
        let nonce = setup.register_item(
            slot.clone(),
            id,
            &ItemAttributes::<DebugApi> {
                item_id: ManagedBuffer::<DebugApi>::new_from_bytes(typeee),
            },
        );

        assert_eq!(nonce, NONCE); // must be the same nonce for all items
    }

    let b_wrapper = &mut setup.blockchain_wrapper;

    let _ = b_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            // instantiate penguin with item
            let penguin_attributes = PenguinAttributes::<DebugApi>::new(&[
                (
                    &ItemSlot::Hat,
                    Item::<DebugApi> {
                        token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_1_IDENTIFIER),
                        nonce: NONCE,
                    },
                ),
                (
                    &ItemSlot::Skin,
                    Item::<DebugApi> {
                        token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_2_IDENTIFIER),
                        nonce: NONCE,
                    },
                ),
            ]);

            let actual = sc.build_url(&penguin_attributes);

            assert!(actual.is_ok());

            let mut expected = ManagedBuffer::new();
            expected.append(&sc.uri().get());
            expected.append(&ManagedBuffer::new_from_bytes(
                SLOT_1.to_bytes::<DebugApi>(),
            ));
            expected.append_bytes(b"_");
            expected.append(&ManagedBuffer::new_from_bytes(ITEM_1_TYPE)); // slot value eg. albino

            expected.append_bytes(b"+");

            expected.append(&ManagedBuffer::new_from_bytes(
                SLOT_2.to_bytes::<DebugApi>(),
            ));
            expected.append_bytes(b"_");
            expected.append(&ManagedBuffer::new_from_bytes(ITEM_2_TYPE)); // slot value eg. albino

            expected.append_bytes(b"/image.png");

            assert_eq!(actual.unwrap(), expected);
        })
        .assert_ok();
}
