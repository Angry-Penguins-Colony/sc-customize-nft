use std::ops::Deref;
use std::u8;

use customize_nft::structs::equippable_nft_attributes::EquippableNftAttributes;
use customize_nft::structs::item::Item;
use customize_nft::structs::item_attributes::ItemAttributes;
use customize_nft::*;
use elrond_wasm::contract_base::ContractBase;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm::types::{
    Address, BigUint, EsdtLocalRole, EsdtTokenPayment, EsdtTokenType, ManagedBuffer, ManagedVec,
    TokenIdentifier,
};
use elrond_wasm_debug::tx_mock::{TxInputESDT, TxResult};
use elrond_wasm_debug::{managed_buffer, managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};

pub const WASM_PATH: &'static str = "sc-customize-nft/output/customize_nft.wasm";

pub const EQUIPPABLE_TOKEN_ID: &[u8] = b"PENG-ae5a";

pub const HAT_TOKEN_ID: &[u8] = b"HAT-a";

#[macro_export]
macro_rules! assert_eq_symetry {
    ($a: expr, $b: expr) => {
        assert!($a == $b, "Failed with (a, b)");
        assert!($b == $a, "Failed with (b, a)");
    };
}

#[macro_export]
macro_rules! assert_ne_symetry {
    ($a: expr, $b: expr) => {
        assert!($a != $b, "Failed with (a, b)");
        assert!($b != $a, "Failed with (b, a)");
    };
}

#[macro_export]
macro_rules! managed_vec [
    ($vec_type: tt, $($e:expr),*) => ({
        let mut _temp = ::std::vec::Vec::<$vec_type>::new();
        $(_temp.push($e);)*
        ManagedVec::<DebugApi, u64>::from(_temp)
    })
];

#[macro_export]
macro_rules! args_set_cid_of {
    ($attr: expr, $cid: expr) => {{
        let mut _val = MultiValueEncoded::new();

        let element = MultiValue2::from(($attr, $cid.clone()));
        _val.push(element);

        _val
    }};
}

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
    pub fn assert_uris(&mut self, token: &[u8], nonce: u64, expected_uris: &[&[u8]]) {
        self.blockchain_wrapper
            .execute_query(&self.cf_wrapper, |sc| {
                let actual_uris = sc
                    .blockchain()
                    .get_esdt_token_data(
                        &sc.blockchain().get_sc_address(),
                        &managed_token_id!(token),
                        nonce,
                    )
                    .uris;

                assert_eq!(
                    actual_uris.len(),
                    expected_uris.len(),
                    "The URIS of {}-{} should have the same length.",
                    std::str::from_utf8(token).unwrap(),
                    nonce
                );

                for (i, expected_uri) in expected_uris.iter().enumerate() {
                    assert_eq!(actual_uris.get(i).deref(), &managed_buffer!(expected_uri));
                }
            })
            .assert_ok();
    }

    pub fn register_and_fill_item(
        &mut self,
        slot: &[u8],
        item_id: &[u8],
        item_nonce: u64,
        attributes: &ItemAttributes<DebugApi>,
    ) {
        self.register_and_fill_items_all_properties(
            slot,
            item_id,
            item_nonce,
            attributes,
            0u64,
            Option::None,
            Option::None,
            Option::None,
            &[],
        );
    }

    pub fn register_and_fill_items_all_properties(
        &mut self,
        slot: &[u8],
        item_id: &[u8],
        item_nonce: u64,
        attributes: &ItemAttributes<DebugApi>,
        royalties: u64,
        creator: Option<&Address>,
        name: Option<&[u8]>,
        hash: Option<&[u8]>,
        uri: &[Vec<u8>],
    ) {
        self.set_all_permissions_on_token(item_id);

        self.blockchain_wrapper
            .execute_tx(
                &self.owner_address,
                &self.cf_wrapper,
                &rust_biguint!(0u64),
                |sc| {
                    let mut managed_items_ids =
                        MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                    managed_items_ids.push(managed_token_id!(item_id));

                    sc.register_item(ManagedBuffer::new_from_bytes(slot), managed_items_ids);
                },
            )
            .assert_ok();

        self.blockchain_wrapper.set_nft_balance_all_properties(
            &self.owner_address,
            &item_id,
            item_nonce,
            &rust_biguint!(2u64),
            &attributes,
            royalties,
            creator,
            name,
            hash,
            uri,
        );

        self.blockchain_wrapper
            .execute_esdt_transfer(
                &self.owner_address,
                &self.cf_wrapper,
                &item_id,
                item_nonce,
                &rust_biguint!(2),
                |sc| {
                    sc.fill();
                },
            )
            .assert_ok();

        println!(
            "Item {:?} created and register with nonce {:x}",
            std::str::from_utf8(item_id).unwrap(),
            item_nonce
        );
    }

    pub fn add_random_item_to_user(&mut self, token_id: &[u8], nonce: u64, quantity: u64) {
        self.blockchain_wrapper.set_nft_balance(
            &self.first_user_address,
            token_id,
            nonce,
            &rust_biguint!(quantity),
            &ItemAttributes::<DebugApi>::random(),
        );
    }

    pub fn set_all_permissions_on_token(&mut self, token_id: &[u8]) {
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

    pub fn create_empty_equippable(&mut self, nonce: u64) {
        DebugApi::dummy();

        self.blockchain_wrapper.set_nft_balance(
            &self.first_user_address,
            EQUIPPABLE_TOKEN_ID,
            nonce,
            &rust_biguint!(1),
            &EquippableNftAttributes::<DebugApi>::empty(),
        );
    }

    pub fn create_equippable_with_registered_item(
        &mut self,
        nonce: u64,
        item_identifier: &[u8],
        item_nonce: u64,
        slot: &[u8],
        attributes: ItemAttributes<DebugApi>,
    ) {
        self.register_and_fill_item(slot, item_identifier, item_nonce, &attributes);

        let attributes = EquippableNftAttributes::<DebugApi>::new(&[Item {
            name: ManagedBuffer::new_from_bytes(
                item_identifier, // sadly, bug in the mock force us to use the item identifier as its name
            ),
            slot: ManagedBuffer::new_from_bytes(slot),
        }]);

        self.blockchain_wrapper.set_nft_balance(
            &self.first_user_address,
            EQUIPPABLE_TOKEN_ID,
            nonce,
            &rust_biguint!(1),
            &attributes,
        );
    }

    pub fn customize(
        &mut self,
        transfers: Vec<TxInputESDT>,
        unequip_slots: &[&[u8]],
    ) -> (Option<u64>, TxResult) {
        let mut opt_sc_result: Option<u64> = Option::None;

        let tx_result = self.blockchain_wrapper.execute_esdt_multi_transfer(
            &self.first_user_address,
            &self.cf_wrapper,
            &transfers,
            |sc| {
                let mut unequip_slots_managed =
                    MultiValueEncoded::<DebugApi, ManagedBuffer<DebugApi>>::new();

                for s in unequip_slots {
                    unequip_slots_managed.push(managed_buffer!(s));
                }

                let result = sc.customize(unequip_slots_managed);

                opt_sc_result = Option::Some(result.clone());
            },
        );

        return (opt_sc_result, tx_result);
    }

    pub fn assert_is_burn(&self, token_id: &[u8], token_nonce: u64) {
        self.assert_is_burn_on(
            token_id,
            token_nonce,
            &self.cf_wrapper.address_ref(),
            "cf_wrapper",
        );
        self.assert_is_burn_on(
            token_id,
            token_nonce,
            &self.first_user_address,
            "first_user_address",
        );
        self.assert_is_burn_on(
            token_id,
            token_nonce,
            &self.second_user_address,
            "second_user_address",
        )
    }

    pub fn assert_is_burn_on(
        &self,
        token_id: &[u8],
        token_nonce: u64,
        address: &Address,
        address_name: &str,
    ) {
        assert_eq!(
            self.blockchain_wrapper
                .get_esdt_balance(address, token_id, token_nonce),
            rust_biguint!(0),
            "{} owns {}-{} while it should be burned.",
            address_name,
            std::str::from_utf8(token_id).unwrap(),
            token_nonce,
        );
    }

    pub fn equip(&mut self, transfers: Vec<TxInputESDT>) -> (Option<u64>, TxResult) {
        let mut opt_sc_result: Option<u64> = Option::None;

        let tx_result = self.blockchain_wrapper.execute_esdt_multi_transfer(
            &self.first_user_address,
            &self.cf_wrapper,
            &transfers,
            |sc| {
                let result =
                    sc.customize(MultiValueEncoded::<DebugApi, ManagedBuffer<DebugApi>>::new());

                opt_sc_result = Option::Some(result);
            },
        );

        return (opt_sc_result, tx_result);
    }
}

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
            sc.init(
                managed_token_id!(EQUIPPABLE_TOKEN_ID),
                managed_buffer!(b"https://ipfs.io/ipfs/"),
                managed_buffer!(b"Equippable #{number}"),
            );
        })
        .assert_ok();
    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    let mut equip_setup = EquipSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        cf_wrapper,
    };

    equip_setup.set_all_permissions_on_token(EQUIPPABLE_TOKEN_ID);

    return equip_setup;
}

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
