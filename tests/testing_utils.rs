use std::u8;

use customize_nft::structs::item::Item;
use customize_nft::structs::item_attributes::ItemAttributes;
use customize_nft::structs::penguin_attributes::PenguinAttributes;
use customize_nft::*;
use elrond_wasm::contract_base::ContractBase;
use elrond_wasm::elrond_codec::multi_types::MultiValue2;
use elrond_wasm::types::{
    Address, BigUint, EsdtLocalRole, EsdtTokenPayment, EsdtTokenType, ManagedBuffer, ManagedVec,
    MultiValueEncoded, SCResult, TokenIdentifier,
};
use elrond_wasm_debug::tx_mock::{TxContextRef, TxInputESDT, TxResult};
use elrond_wasm_debug::{managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};

#[allow(dead_code)]
const WASM_PATH: &'static str = "sc-customize-nft/output/customize_nft.wasm";

#[allow(dead_code)]
pub const PENGUIN_TOKEN_ID: &[u8] = b"PENG-ae5a";
#[allow(dead_code)]
pub const HAT_TOKEN_ID: &[u8] = b"HAT-a";
#[allow(dead_code)]
pub const HAT_2_TOKEN_ID: &[u8] = b"HAT-b";

pub const INIT_NONCE: u64 = 65535u64;

#[allow(dead_code)]
pub struct EquipSetup<CrowdfundingObjBuilder>
where
    CrowdfundingObjBuilder: 'static + Copy + Fn() -> customize_nft::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub cf_wrapper:
        ContractObjWrapper<customize_nft::ContractObj<DebugApi>, CrowdfundingObjBuilder>,
}

impl<CrowdfundingObjBuilder> EquipSetup<CrowdfundingObjBuilder>
where
    CrowdfundingObjBuilder: 'static + Copy + Fn() -> customize_nft::ContractObj<DebugApi>,
{
    #[allow(dead_code)]
    pub fn add_item(
        &mut self,
        token: &[u8],
        nonce: u64,
        quantity: u64,
        attributes: &ItemAttributes<DebugApi>,
    ) {
        self.blockchain_wrapper.set_nft_balance(
            &self.first_user_address,
            token,
            nonce,
            &rust_biguint!(quantity),
            attributes,
        );
    }

    #[allow(dead_code)]
    pub fn register_item(
        &mut self,
        item_type: ManagedBuffer<DebugApi>,
        item_id: &[u8],
        attributes: &ItemAttributes<DebugApi>,
    ) -> u64 {
        return self.register_item_all_properties(
            item_type,
            item_id,
            attributes,
            0u64,
            Option::None,
            Option::None,
            Option::None,
            &[],
        );
    }

    #[allow(dead_code)]
    pub fn register_item_all_properties(
        &mut self,
        item_type: ManagedBuffer<DebugApi>,
        item_id: &[u8],
        attributes: &ItemAttributes<DebugApi>,
        royalties: u64,
        creator: Option<&Address>,
        name: Option<&[u8]>,
        hash: Option<&[u8]>,
        uri: &[Vec<u8>],
    ) -> u64 {
        self.blockchain_wrapper.set_nft_balance_all_properties(
            &self.cf_wrapper.address_ref(),
            &item_id,
            INIT_NONCE,
            &rust_biguint!(2u64),
            &attributes,
            royalties,
            creator,
            name,
            hash,
            uri,
        );

        self.set_all_permissions_on_token(item_id);

        self.blockchain_wrapper
            .execute_tx(
                &self.owner_address,
                &self.cf_wrapper,
                &rust_biguint!(0u64),
                |sc| {
                    let data = sc.blockchain().get_esdt_token_data(
                        &sc.blockchain().get_sc_address(),
                        &TokenIdentifier::from_esdt_bytes(item_id),
                        INIT_NONCE,
                    );

                    println!("Name is {:?}", data.name);

                    let mut managed_items_ids =
                        MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                    managed_items_ids.push(managed_token_id!(item_id));

                    let result = sc.register_item(item_type, managed_items_ids);

                    if let SCResult::Err(err) = result {
                        panic!(
                            "register_item {:?} failed: {:?}",
                            std::str::from_utf8(&item_id).unwrap(),
                            std::str::from_utf8(&err.as_bytes()).unwrap(),
                        );
                    }

                    assert_eq!(result, SCResult::Ok(()));
                },
            )
            .assert_ok();

        println!(
            "Item {:?} created and register with nonce {:x}",
            name, INIT_NONCE
        );

        return INIT_NONCE;
    }

    #[allow(dead_code)]
    pub fn add_random_item_to_user(&mut self, token_id: &[u8], nonce: u64, quantity: u64) {
        self.blockchain_wrapper.set_nft_balance(
            &self.first_user_address,
            token_id,
            nonce,
            &rust_biguint!(quantity),
            &ItemAttributes::<DebugApi>::random(),
        );
    }

    fn set_all_permissions_on_token(&mut self, token_id: &[u8]) {
        let contract_roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftBurn,
            EsdtLocalRole::NftAddQuantity,
        ];
        self.blockchain_wrapper.set_esdt_local_roles(
            self.cf_wrapper.address_ref(),
            token_id,
            &contract_roles,
        );
    }

    #[allow(dead_code)]
    pub fn create_penguin_empty(&mut self, penguin_nonce: u64) {
        self.blockchain_wrapper.set_nft_balance(
            &self.first_user_address,
            PENGUIN_TOKEN_ID,
            penguin_nonce,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::empty(),
        );
    }

    #[allow(dead_code)]
    pub fn create_penguin_with_registered_item(
        &mut self,
        penguin_nonce: u64,
        item_identifier: &[u8],
        item_nonce: u64,
        slot: ManagedBuffer<DebugApi>,
        attributes: ItemAttributes<DebugApi>,
    ) {
        let _ = self.register_item(slot.clone(), item_identifier, &attributes);

        self.blockchain_wrapper.set_nft_balance(
            &self.cf_wrapper.address_ref(),
            &item_identifier,
            item_nonce,
            &rust_biguint!(2u64),
            &attributes,
        );

        let attributes = PenguinAttributes::new(&[(
            &slot,
            Item {
                token: TokenIdentifier::<DebugApi>::from_esdt_bytes(item_identifier),
                nonce: item_nonce,
                name: ManagedBuffer::new_from_bytes(b"item name"),
            },
        )]);

        self.blockchain_wrapper.set_nft_balance(
            &self.first_user_address,
            PENGUIN_TOKEN_ID,
            penguin_nonce,
            &rust_biguint!(1),
            &attributes,
        );
    }

    #[allow(dead_code)]
    pub fn customize(
        &mut self,
        transfers: Vec<TxInputESDT>,
        slot: ManagedBuffer<DebugApi>,
    ) -> (SCResult<u64>, TxResult) {
        let mut opt_sc_result: Option<SCResult<u64>> = Option::None;

        let tx_result = self.blockchain_wrapper.execute_esdt_multi_transfer(
            &self.first_user_address,
            &self.cf_wrapper,
            &transfers,
            |sc| {
                let mut managed_slots =
                    MultiValueEncoded::<DebugApi, ManagedBuffer<DebugApi>>::new();
                managed_slots.push(slot.clone());

                let result = sc.customize(sc.call_value().all_esdt_transfers(), managed_slots);

                opt_sc_result = Option::Some(result.clone());
            },
        );

        match opt_sc_result {
            Option::Some(sc_result) => return (sc_result, tx_result),
            Option::None => return (SCResult::Err("".into()), tx_result),
        }
    }

    #[allow(dead_code)]
    pub fn assert_is_burn(&mut self, token_id: &[u8], token_nonce: u64) {
        assert_eq!(
            self.blockchain_wrapper.get_esdt_balance(
                &self.first_user_address,
                token_id,
                token_nonce
            ),
            rust_biguint!(0)
        );

        assert_eq!(
            self.blockchain_wrapper.get_esdt_balance(
                &self.second_user_address,
                token_id,
                token_nonce
            ),
            rust_biguint!(0)
        );

        assert_eq!(
            self.blockchain_wrapper.get_esdt_balance(
                self.cf_wrapper.address_ref(),
                token_id,
                token_nonce
            ),
            rust_biguint!(0)
        );
    }

    #[allow(dead_code)]
    pub fn equip(&mut self, transfers: Vec<TxInputESDT>) -> (SCResult<u64>, TxResult) {
        let mut opt_sc_result: Option<SCResult<u64>> = Option::None;

        let tx_result = self.blockchain_wrapper.execute_esdt_multi_transfer(
            &self.first_user_address,
            &self.cf_wrapper,
            &transfers,
            |sc| {
                let result = sc.customize(
                    sc.call_value().all_esdt_transfers(),
                    MultiValueEncoded::<DebugApi, ManagedBuffer<DebugApi>>::new(),
                );

                opt_sc_result = Option::Some(result.clone());
            },
        );

        match opt_sc_result {
            Option::Some(sc_result) => return (sc_result, tx_result),
            Option::None => return (SCResult::Err("".into()), tx_result),
        }
    }
}

#[allow(dead_code)]
pub fn setup<TObjBuilder>(cf_builder: TObjBuilder) -> EquipSetup<TObjBuilder>
where
    TObjBuilder: 'static + Copy + Fn() -> customize_nft::ContractObj<DebugApi>,
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
    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            let result = sc.init(managed_token_id!(PENGUIN_TOKEN_ID));
            assert_eq!(result, SCResult::Ok(()));
        })
        .assert_ok();
    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    DebugApi::dummy();

    let mut equip_setup = EquipSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        cf_wrapper,
    };

    equip_setup.set_all_permissions_on_token(PENGUIN_TOKEN_ID);

    return equip_setup;
}

#[allow(dead_code)]
pub fn verbose_log_if_error<T>(result: &SCResult<T>, message: String) {
    if let SCResult::Err(err) = &*result {
        panic!(
            "{} | failed {:?}",
            message,
            std::str::from_utf8(&err.as_bytes()).unwrap(),
        );
    }
}

#[allow(dead_code)]
pub fn create_managed_items_to_equip(
    tokens: &[(&[u8], u64)],
) -> MultiValueEncoded<
    TxContextRef,
    MultiValue2<elrond_wasm::types::TokenIdentifier<TxContextRef>, u64>,
> {
    let mut managed_items_to_equip =
        MultiValueEncoded::<DebugApi, MultiValue2<TokenIdentifier<DebugApi>, u64>>::new();

    for (token_id, nonce) in tokens {
        managed_items_to_equip.push(MultiValue2((
            TokenIdentifier::<DebugApi>::from_esdt_bytes(token_id.clone()),
            nonce.clone(),
        )));
    }

    return managed_items_to_equip;
}

#[allow(dead_code)]
pub fn create_paymens_and_esdt_transfers(
    tokens: &[(&[u8], u64, EsdtTokenType)],
) -> (
    Vec<TxInputESDT>,
    ManagedVec<DebugApi, EsdtTokenPayment<DebugApi>>,
) {
    // remove EsdtTokenType from tokens
    let mut tokens_without_type = Vec::new();
    for (token_id, nonce, _) in tokens {
        tokens_without_type.push((token_id.clone(), nonce.clone()));
    }

    return (
        create_esdt_transfers(tokens_without_type.as_slice()),
        create_payments(tokens),
    );
}

#[allow(dead_code)]
pub fn create_esdt_transfers(tokens: &[(&[u8], u64)]) -> Vec<TxInputESDT> {
    let mut transfers = Vec::new();

    for (token_id, nonce) in tokens {
        transfers.push(TxInputESDT {
            token_identifier: token_id.to_vec(),
            nonce: nonce.clone(),
            value: rust_biguint!(1u64),
        })
    }

    return transfers;
}

#[allow(dead_code)]
pub fn create_payments(
    tokens: &[(&[u8], u64, EsdtTokenType)],
) -> ManagedVec<DebugApi, EsdtTokenPayment<DebugApi>> {
    let mut payments = ManagedVec::<DebugApi, EsdtTokenPayment<DebugApi>>::new();

    for (token_id, nonce, _) in tokens {
        let payment = EsdtTokenPayment::new(
            TokenIdentifier::<DebugApi>::from_esdt_bytes(token_id.to_vec()),
            nonce.clone(),
            BigUint::from(1u64),
        );

        payments.push(payment)
    }

    return payments;
}

// TODO: register item (arg = slot)
// TODO: add quantity (arg = quantity)
