use elrond_wasm::types::{MultiArg2, SCResult, TokenIdentifier};
use elrond_wasm_debug::DebugApi;
use equip_penguin::{item_slot::ItemSlot, penguins_attributes::PenguinAttributes};

#[test]
fn test_is_empty_while_not_empty() {
    execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let penguin = PenguinAttributes {
            hat: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"HAT-a"), 1),
        };

        assert_eq!(penguin.is_slot_empty(slot), Result::Ok(false));
    });
}

#[test]
fn test_is_empty_while_empty() {
    execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let penguin = PenguinAttributes {
            hat: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b""), 1),
        };

        assert_eq!(penguin.is_slot_empty(slot), Result::Ok(true));
    });
}

#[test]
fn set_item_should_set_item() {
    execute_for_all_slot(|slot| {
        DebugApi::dummy();

        let mut penguin = PenguinAttributes {
            hat: (TokenIdentifier::<DebugApi>::from_esdt_bytes(b"HAT-a"), 1),
        };

        let token = b"ITEM-b";
        let managed_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(token);
        let nonce = 1;

        let result = penguin.empty_slot(&ItemSlot::Hat);
        assert_eq!(result, Result::Ok(()));

        let result = penguin.set_item(&slot, managed_token.clone(), nonce);
        assert_eq!(result, Result::Ok(()));

        assert_eq!(
            penguin.get_item(slot).unwrap().into_tuple(),
            (managed_token, nonce)
        );
    });
}

fn execute_for_all_slot(execute: fn(&ItemSlot) -> ()) {
    execute(&ItemSlot::Hat);
    // for slot in ItemSlot::VALUES.iter() {
    //     execute(slot);
    // }
}
