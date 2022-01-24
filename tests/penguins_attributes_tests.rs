use elrond_wasm::types::{ManagedBuffer, MultiArg2, SCResult, TokenIdentifier};
use elrond_wasm_debug::DebugApi;
use equip_penguin::{item_slot::ItemSlot, penguins_attributes::PenguinAttributes};

#[test]
fn is_empty_while_not_empty() {
    execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let penguin = PenguinAttributes {
            hat: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"HAT-a"), 1),
            background: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(b"BACKGROUND-a"),
                1,
            ),
            skin: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"SKIN-a"), 1),
            chain: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"CHAIN-a"), 1),
            beak: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"BEAK-a"), 1),
            weapon: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"WEAPON-a"), 1),
            clothes: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(b"CLOTHES-a"),
                1,
            ),
            eye: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"EYE-a"), 1),
        };

        assert_eq!(penguin.is_slot_empty(slot), Result::Ok(false));
    });
}

#[test]
fn is_empty_while_empty() {
    execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let penguin = PenguinAttributes {
            hat: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b""), 1),
            ..Default::default()
        };

        assert_eq!(penguin.is_slot_empty(slot), Result::Ok(true));
    });
}

#[test]
fn set_item_on_empty_slot() {
    execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes {
            hat: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b""), 0),
            ..Default::default()
        };

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
    execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes {
            hat: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"HAT-a"), 1),
            background: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(b"BACKGROUND-a"),
                1,
            ),
            skin: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"SKIN-a"), 1),
            chain: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"CHAIN-a"), 1),
            beak: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"BEAK-a"), 1),
            weapon: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"WEAPON-a"), 1),
            clothes: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(b"CLOTHES-a"),
                1,
            ),
            eye: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"EYE-a"), 1),
            ..Default::default()
        };

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
    execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes {
            hat: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b""), 1),
            ..Default::default()
        };

        let result = penguin.empty_slot(&slot);
        assert_eq!(result, Result::Ok(()));
    });
}

#[test]
fn empty_slot_while_slot_is_not_empty() {
    execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes {
            hat: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"HAT-aaaa"), 1),
            ..Default::default()
        };

        let result = penguin.empty_slot(&slot);
        assert_eq!(result, Result::Ok(()));
    });
}

fn execute_for_all_slot(execute: fn(&ItemSlot) -> ()) {
    // execute(&ItemSlot::Hat);
    for slot in ItemSlot::VALUES.iter() {
        execute(slot);
    }
}
