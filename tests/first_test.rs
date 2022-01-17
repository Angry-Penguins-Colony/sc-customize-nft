use elrond_wasm::types::ManagedVec;
use elrond_wasm::types::{ManagedBuffer, ManagedVarArgs};
use elrond_wasm_debug::{testing_framework::*, DebugApi};
use equip_penguin::*;

// const WASM_PATH: &'static str = "sc-equip-penguin/output/equip_penguin.wasm";

// const PENGUIN_ID: &'static str = "PENG-ae5a";
const ITEM_TYPE_HAT: &'static str = "hat";
const HAT_ID: &'static str = "HAT-7e8f";

#[test]
fn test_register_item() {
    let contract = equip_penguin::contract_obj(DebugApi::dummy());
    let _ = contract.init();

    let item_type = ManagedBuffer::<DebugApi>::new_from_bytes(ITEM_TYPE_HAT.as_bytes());
    let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_ID.as_bytes());

    let mut items_ids = ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
    items_ids.push(hat_token.clone());

    contract.register_item(&item_type, items_ids);

    let mut o = ManagedVec::new();
    o.push(hat_token);

    match contract.items_types().get(&item_type) {
        Some(items_ids) => {
            assert_eq!(items_ids, o);
        }
        None => {
            panic!("no item_type found");
        }
    }
}

#[test]
fn test_get_item() {}
