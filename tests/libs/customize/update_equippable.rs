use crate::testing_utils::EquipSetup;
use customize_nft::{
    libs::customize::CustomizeModule, structs::equippable_attributes::EquippableAttributes, Equip,
};
use elrond_wasm::{contract_base::ContractBase, types::BigUint};
use elrond_wasm_debug::{managed_buffer, managed_token_id, rust_biguint, DebugApi};

use crate::testing_utils;
use crate::testing_utils::New;
use crate::testing_utils::EQUIPPABLE_TOKEN_ID;

#[test]
fn use_previous_royalties() {
    const MINT_NONCE: u64 = 555;
    const MINT_ROYALTIES: u64 = 100u64;

    let get_attributes = || EquippableAttributes::new(&[]);

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.setup_test(MINT_NONCE, MINT_ROYALTIES, &get_attributes);
    setup.update_equippable_and_assert_royalties(MINT_NONCE, &get_attributes, MINT_ROYALTIES);
}

#[test]
fn use_overrided_royalties() {
    const MINT_NONCE: u64 = 555;
    const MINT_ROYALTIES: u64 = 100u64;
    const OVERRIDE_ROYALTIES: u64 = 250u64;

    assert_ne!(
        MINT_ROYALTIES, OVERRIDE_ROYALTIES,
        "The testing royalties must be different"
    );

    let get_attributes = || EquippableAttributes::new(&[]);

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.setup_test(MINT_NONCE, MINT_ROYALTIES, &get_attributes);
    setup.override_royalties(OVERRIDE_ROYALTIES);
    setup.update_equippable_and_assert_royalties(MINT_NONCE, &get_attributes, OVERRIDE_ROYALTIES);
}

impl<CrowdfundingObjBuilder> EquipSetup<CrowdfundingObjBuilder>
where
    CrowdfundingObjBuilder: 'static + Copy + Fn() -> customize_nft::ContractObj<DebugApi>,
{
    fn setup_test(
        &mut self,
        mint_nonce: u64,
        mint_royalties: u64,
        mint_attributes: &dyn Fn() -> EquippableAttributes<DebugApi>,
    ) {
        assert_ne!(mint_nonce, 1, "The test doesn't work with nonce 1");

        DebugApi::dummy();

        self.blockchain_wrapper.set_nft_balance_all_properties(
            &self.cf_wrapper.address_ref(),
            EQUIPPABLE_TOKEN_ID,
            mint_nonce,
            &rust_biguint!(1),
            &mint_attributes(),
            mint_royalties,
            Option::None,
            Option::None,
            Option::None,
            &[],
        );

        self.enqueue_and_set_cid_of(
            &|| {
                return (mint_attributes(), managed_buffer!(EQUIPPABLE_TOKEN_ID));
            },
            b"default uri",
        );
    }

    fn override_royalties(&mut self, royalties: u64) {
        self.blockchain_wrapper
            .execute_tx(
                &self.owner_address,
                &self.cf_wrapper,
                &rust_biguint!(0),
                |sc| {
                    sc.override_royalties(BigUint::from(royalties));
                },
            )
            .assert_ok();
    }
    fn update_equippable_and_assert_royalties(
        &mut self,
        mint_nonce: u64,
        get_attributes: &dyn Fn() -> EquippableAttributes<DebugApi>,
        expected_royalties: u64,
    ) {
        self.blockchain_wrapper
            .execute_tx(
                &self.owner_address,
                &self.cf_wrapper,
                &rust_biguint!(0),
                |sc| {
                    let minted_nonce = sc.update_equippable(mint_nonce, &get_attributes());

                    let minted_nft = sc.blockchain().get_esdt_token_data(
                        &sc.blockchain().get_sc_address(),
                        &managed_token_id!(EQUIPPABLE_TOKEN_ID),
                        minted_nonce,
                    );

                    assert_eq!(minted_nft.royalties, expected_royalties);
                },
            )
            .assert_ok();
    }
}
