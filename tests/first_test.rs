use elrond_wasm::api::ESDT_NFT_CREATE_FUNC_NAME;
use elrond_wasm::types::{
    Address, EsdtLocalRole, ManagedBuffer, ManagedVarArgs, MultiArg2, SCResult,
};
use elrond_wasm::types::{ManagedVec, OptionalResult};
use elrond_wasm_debug::tx_mock::{TxContextRef, TxInputESDT};
use elrond_wasm_debug::{managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::*;

const WASM_PATH: &'static str = "sc-equip-penguin/output/equip_penguin.wasm";

const PENGUIN_TOKEN_ID: &[u8] = b"PENG-ae5a";
const ITEM_TYPE_HAT: &[u8] = b"hat";
const HAT_TOKEN_ID: &[u8] = b"HAT-7e8f";

struct EquipSetup<CrowdfundingObjBuilder>
where
    CrowdfundingObjBuilder: 'static + Copy + Fn(DebugApi) -> equip_penguin::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub cf_wrapper:
        ContractObjWrapper<equip_penguin::ContractObj<DebugApi>, CrowdfundingObjBuilder>,
}

// This is the nonce for the NFTs not generated from the contract but from the setup
// Because, the contract will generate an NFT with the nonce '1', we don't want the INIT_NONCE to be '1'
const INIT_NONCE: u64 = 65535;

// create NFT on blockchain wrapper
#[test]
fn test_equip() {
    let mut setup = setup(equip_penguin::contract_obj);

    let mut b_wrapper = &mut setup.blockchain_wrapper;

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
    let mut setup = setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);

            match sc.get_item_type(&hat_token) {
                OptionalResult::Some(item_type) => {
                    assert_eq!(
                        item_type,
                        ManagedBuffer::<DebugApi>::new_from_bytes(ITEM_TYPE_HAT)
                    );
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
    let mut setup = setup(equip_penguin::contract_obj);

    register_item(&mut setup, ITEM_TYPE_HAT, HAT_TOKEN_ID);

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let managed_item_type = ManagedBuffer::<DebugApi>::new_from_bytes(ITEM_TYPE_HAT);

            let managed_token_id = TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID);
            let mut managed_items_ids = ManagedVec::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id.clone());

            match sc.items_types().get(&managed_item_type) {
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

fn setup<TObjBuilder>(cf_builder: TObjBuilder) -> EquipSetup<TObjBuilder>
where
    TObjBuilder: 'static + Copy + Fn(DebugApi) -> equip_penguin::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let first_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let second_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    // deploy contract
    blockchain_wrapper.execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
        let result = sc.init();
        assert_eq!(result, SCResult::Ok(()));

        StateChange::Commit
    });
    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    DebugApi::dummy();

    // set NFTs balance
    let none_value = TokenIdentifier::<DebugApi>::from_esdt_bytes(b"NONE-000000");

    let nft_attributes = PenguinAttributes {
        hat: none_value.clone(),
        // background: none_value.clone(),
    };

    blockchain_wrapper.set_nft_balance(
        &first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &nft_attributes,
    );

    blockchain_wrapper.set_nft_balance(
        &first_user_address,
        HAT_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &ItemAttributes {},
    );

    blockchain_wrapper.set_esdt_local_roles(
        cf_wrapper.address_ref(),
        PENGUIN_TOKEN_ID,
        &[EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn],
    );

    blockchain_wrapper.set_esdt_local_roles(
        cf_wrapper.address_ref(),
        HAT_TOKEN_ID,
        &[EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn],
    );

    let mut equip_setup = EquipSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        cf_wrapper,
    };

    // register items
    register_item(&mut equip_setup, ITEM_TYPE_HAT, HAT_TOKEN_ID);

    equip_setup
}

fn register_item<EquipObjBuilder>(
    setup: &mut EquipSetup<EquipObjBuilder>,
    item_type: &[u8],
    item_id: &[u8],
) where
    EquipObjBuilder: 'static + Copy + Fn(DebugApi) -> equip_penguin::ContractObj<DebugApi>,
{
    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let managed_token_id = TokenIdentifier::<DebugApi>::from_esdt_bytes(item_id);
            let mut managed_items_ids =
                ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id.clone());

            let result = sc.register_item(
                &ManagedBuffer::<DebugApi>::from(item_type),
                managed_items_ids,
            );
            assert_eq!(result, SCResult::Ok(()));

            StateChange::Commit
        },
    );
}

// fn register_items(
//     contract: &ContractObjWrapper<TxContextRef>,
//     item_type: &str,
//     token_id: &str,
// ) -> (
//     ManagedBuffer<elrond_wasm_debug::tx_mock::TxContextRef>,
//     TokenIdentifier<elrond_wasm_debug::tx_mock::TxContextRef>,
// ) {
//     let item_type = ManagedBuffer::<DebugApi>::new_from_bytes(item_type.as_bytes());
//     let hat_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(token_id.as_bytes());
//     let mut items_ids = ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
//     items_ids.push(hat_token.clone());
//     contract.register_item(&item_type, items_ids);
//     (item_type, hat_token)
// }
