use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::DebugApi;
use equip_penguin::structs::{item::Item, penguin_attributes::PenguinAttributes};

mod utils;

#[test]
fn is_empty_while_not_empty() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let penguin = PenguinAttributes::<DebugApi>::new(&[(
            slot,
            Item {
                token: TokenIdentifier::from_esdt_bytes(b"ITEM-a"),
                nonce: 0,
                name: ManagedBuffer::new_from_bytes(b"item name"),
            },
        )]);

        assert_eq!(penguin.is_slot_empty(slot), false);
    });
}

#[test]
fn is_empty_while_empty() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let penguin = PenguinAttributes::<DebugApi>::empty();

        assert_eq!(penguin.is_slot_empty(slot), true);
    });
}

#[test]
fn set_item_on_empty_slot() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes::<DebugApi>::empty();

        let token = b"ITEM-b";
        let managed_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(token);
        let nonce = 1;

        let result = penguin.set_item(
            &slot,
            Option::Some(Item {
                token: managed_token.clone(),
                nonce: nonce,
                name: ManagedBuffer::new_from_bytes(b"item name"),
            }),
        );
        assert_eq!(result, Result::Ok(()));

        let item = penguin.get_item(slot).unwrap();

        assert_eq!(item.token, managed_token);
        assert_eq!(item.nonce, nonce);
    });
}

#[test]
fn set_item_on_not_empty_slot() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes::<DebugApi>::new(&[(
            slot,
            Item {
                token: TokenIdentifier::from_esdt_bytes(b"ITEM-a"),
                nonce: 0,
                name: ManagedBuffer::new_from_bytes(b"item name"),
            },
        )]);

        let token = b"ITEM-b";
        let managed_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(token);
        let nonce = 1;

        let result = penguin.set_item(
            &slot,
            Option::Some(Item {
                token: managed_token.clone(),
                nonce: nonce,
                name: ManagedBuffer::new_from_bytes(b"item name"),
            }),
        );
        assert_eq!(
            result,
            Result::Err(ManagedBuffer::new_from_bytes(
                b"The slot is not empty. Please free it, before setting an item.",
            ))
        );
    });
}

#[test]
fn empty_slot_while_slot_is_empty() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes::<DebugApi>::new(&[(
            slot,
            Item {
                token: TokenIdentifier::from_esdt_bytes(b""),
                nonce: 0,
                name: ManagedBuffer::new_from_bytes(b"item name"),
            },
        )]);

        let result = penguin.set_empty_slot(&slot);
        assert_eq!(result, Result::Ok(()));
    });
}

#[test]
fn empty_slot_while_slot_is_not_empty() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes::<DebugApi>::new(&[(
            slot,
            Item {
                token: TokenIdentifier::from_esdt_bytes(b"HAT-aaaa"),
                nonce: 0,
                name: ManagedBuffer::new_from_bytes(b"item name"),
            },
        )]);

        let result = penguin.set_empty_slot(&slot);
        assert_eq!(result, Result::Ok(()));
    });
}
