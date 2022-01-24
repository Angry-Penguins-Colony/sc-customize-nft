use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::DebugApi;
use equip_penguin::penguins_attributes::PenguinAttributes;

mod utils;

#[test]
fn is_empty_while_not_empty() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let penguin = PenguinAttributes::<DebugApi>::new(&[(
            slot,
            TokenIdentifier::from_esdt_bytes(b"ITEM-a"),
            0,
        )]);

        assert_eq!(penguin.is_slot_empty(slot), Result::Ok(false));
    });
}

#[test]
fn is_empty_while_empty() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let penguin =
            PenguinAttributes::<DebugApi>::new(&[(slot, TokenIdentifier::from_esdt_bytes(b""), 0)]);

        assert_eq!(penguin.is_slot_empty(slot), Result::Ok(true));
    });
}

#[test]
fn set_item_on_empty_slot() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin =
            PenguinAttributes::<DebugApi>::new(&[(slot, TokenIdentifier::from_esdt_bytes(b""), 0)]);

        let token = b"ITEM-b";
        let managed_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(token);
        let nonce = 1;

        let result = penguin.set_item(&slot, managed_token.clone(), nonce);
        assert_eq!(result, Result::Ok(()));

        assert_eq!(
            penguin.get_item(slot).unwrap().into_tuple(),
            (managed_token, nonce)
        );
    });
}

#[test]
fn set_item_on_not_empty_slot() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes::<DebugApi>::new(&[(
            slot,
            TokenIdentifier::from_esdt_bytes(b"ITEM-a"),
            0,
        )]);

        let token = b"ITEM-b";
        let managed_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(token);
        let nonce = 1;

        let result = penguin.set_item(&slot, managed_token.clone(), nonce);
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

        let mut penguin =
            PenguinAttributes::<DebugApi>::new(&[(slot, TokenIdentifier::from_esdt_bytes(b""), 0)]);

        let result = penguin.empty_slot(&slot);
        assert_eq!(result, Result::Ok(()));
    });
}

#[test]
fn empty_slot_while_slot_is_not_empty() {
    utils::execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes::<DebugApi>::new(&[(
            slot,
            TokenIdentifier::from_esdt_bytes(b"HAT-aaaa"),
            0,
        )]);

        let result = penguin.empty_slot(&slot);
        assert_eq!(result, Result::Ok(()));
    });
}
