use elrond_wasm::types::{ManagedBuffer, ManagedVarArgs};
use elrond_wasm::types::{ManagedVec, OptionalResult};
use elrond_wasm_debug::tx_mock::TxContextRef;
use elrond_wasm_debug::{testing_framework::*, DebugApi};
use equip_penguin::*;

// const WASM_PATH: &'static str = "sc-equip-penguin/output/equip_penguin.wasm";

// const PENGUIN_ID: &'static str = "PENG-ae5a";
const ITEM_TYPE_HAT: &'static str = "hat";
const HAT_ID: &'static str = "HAT-7e8f";

#[test]
fn test_get_item() {
    let contract = setup();

    let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_ID.as_bytes());

    match contract.get_item_type(&hat_token) {
        OptionalResult::Some(item_type) => {
            assert_eq!(
                item_type,
                ManagedBuffer::<DebugApi>::new_from_bytes(ITEM_TYPE_HAT.as_bytes())
            );
        }
        OptionalResult::None => {
            panic!("no item_type found");
        }
    }

    let not_existing_token =
        TokenIdentifier::<DebugApi>::from_esdt_bytes("PAR ALLAH PELO".as_bytes());

    match contract.get_item_type(&not_existing_token) {
        OptionalResult::Some(_) => {
            panic!("item_type found");
        }
        OptionalResult::None => {}
    }
}

#[test]
fn test_register_item() {
    let contract = deploy();

    let (item_type, hat_token) = register_items(&contract, ITEM_TYPE_HAT, HAT_ID);

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

fn setup() -> equip_penguin::ContractObj<TxContextRef> {
    let contract = deploy();
    let (_, _) = register_items(&contract, ITEM_TYPE_HAT, HAT_ID);

    contract
}

fn register_items(
    contract: &ContractObj<TxContextRef>,
    item_type: &str,
    token_id: &str,
) -> (
    ManagedBuffer<elrond_wasm_debug::tx_mock::TxContextRef>,
    TokenIdentifier<elrond_wasm_debug::tx_mock::TxContextRef>,
) {
    let item_type = ManagedBuffer::<DebugApi>::new_from_bytes(item_type.as_bytes());
    let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(token_id.as_bytes());
    let mut items_ids = ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
    items_ids.push(hat_token.clone());
    contract.register_item(&item_type, items_ids);
    (item_type, hat_token)
}

fn deploy() -> equip_penguin::ContractObj<TxContextRef> {
    let contract = equip_penguin::contract_obj(DebugApi::dummy());
    let _ = contract.init();
    contract
}
