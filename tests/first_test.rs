use elrond_wasm::types::{ManagedVarArgs, MultiArg2, SCResult};
use elrond_wasm::types::{ManagedVec, OptionalResult};
use elrond_wasm_debug::testing_framework::*;
use elrond_wasm_debug::tx_mock::TxInputESDT;
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::*;

mod utils;

const PENGUIN_TOKEN_ID: &[u8] = utils::utils::PENGUIN_TOKEN_ID;
const HAT_TOKEN_ID: &[u8] = utils::utils::HAT_TOKEN_ID;
const INIT_NONCE: u64 = utils::utils::INIT_NONCE;

// create NFT on blockchain wrapper
#[test]
fn test_equip() {
    let mut setup = utils::utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    let mut transfers = Vec::new();
    transfers.push(TxInputESDT {
        token_identifier: PENGUIN_TOKEN_ID.to_vec(),
        nonce: INIT_NONCE,
        value: rust_biguint!(1),
    });
    transfers.push(TxInputESDT {
        token_identifier: HAT_TOKEN_ID.to_vec(),
        nonce: INIT_NONCE,
        value: rust_biguint!(1),
    });

    let none_value = TokenIdentifier::<DebugApi>::from_esdt_bytes(b"NONE-000000");

    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        HAT_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &ItemAttributes {},
    );

    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &PenguinAttributes {
            hat: none_value.clone(),
        },
    );

    b_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        &transfers,
        |sc| {
            let managed_penguin_token_id =
                TokenIdentifier::<DebugApi>::from_esdt_bytes(PENGUIN_TOKEN_ID);

            let managed_hat_token_id = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);
            let mut managed_items_to_equip =
                ManagedVarArgs::<DebugApi, MultiArg2<TokenIdentifier<DebugApi>, u64>>::new();
            managed_items_to_equip.push(MultiArg2((managed_hat_token_id, INIT_NONCE)));

            let result = sc.equip(
                &managed_penguin_token_id,
                INIT_NONCE,
                managed_items_to_equip,
            );

            assert_eq!(result, SCResult::Ok(1u64));

            StateChange::Commit
        },
    );

    // generated penguin has been sent
    b_wrapper.check_nft_balance(
        &setup.cf_wrapper.address_ref(),
        PENGUIN_TOKEN_ID,
        1u64,
        &rust_biguint!(0),
        &PenguinAttributes {
            hat: TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
        },
    );

    // the transfered penguin has been burn
    b_wrapper.check_nft_balance(
        &setup.cf_wrapper.address_ref(),
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(0),
        &PenguinAttributes {
            hat: none_value.clone(),
        },
    );

    // the transfered penguin has not been sent back
    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(0),
        &PenguinAttributes {
            hat: none_value.clone(),
        },
    );

    // the NEW penguin has been received
    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        1u64,
        &rust_biguint!(1),
        &PenguinAttributes {
            hat: TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
        },
    );

    // the transfered hat has been burn
    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        HAT_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(0),
        &ItemAttributes {},
    );
}

#[test]
fn test_get_item() {
    let mut setup = utils::utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);

            match sc.get_item_type(&hat_token) {
                OptionalResult::Some(item_type) => {
                    assert_eq!(item_type, ItemSlot::Hat);
                }
                OptionalResult::None => {
                    panic!("no item_type found");
                }
            }

            StateChange::Commit
        },
    );

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let not_existing_token =
                TokenIdentifier::<DebugApi>::from_esdt_bytes("PAR ALLAH PELO".as_bytes());

            match sc.get_item_type(&not_existing_token) {
                OptionalResult::Some(_) => {
                    panic!("item_type found");
                }
                OptionalResult::None => {}
            }

            StateChange::Commit
        },
    );
} // */
#[test]
fn test_register_item() {
    let mut setup = utils::utils::setup(equip_penguin::contract_obj);

    utils::utils::register_item(&mut setup, ItemSlot::Hat, HAT_TOKEN_ID);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let managed_token_id = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);
            let mut managed_items_ids = ManagedVec::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id.clone());

            match sc.items_types().get(&ItemSlot::Hat) {
                Some(output_items) => {
                    assert_eq!(output_items, managed_items_ids);
                }
                None => {
                    panic!("no item_type found");
                }
            }

            StateChange::Commit
        },
    );
}
