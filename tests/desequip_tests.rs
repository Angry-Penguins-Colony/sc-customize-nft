use elrond_wasm::types::{ManagedVarArgs, SCResult};
use elrond_wasm_debug::testing_framework::*;
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::item::Item;
use equip_penguin::item_slot::ItemSlot;
use equip_penguin::penguin_attributes::PenguinAttributes;
use equip_penguin::*;
use utils::{create_esdt_transfers, execute_for_all_slot};

mod utils;

const PENGUIN_TOKEN_ID: &[u8] = utils::PENGUIN_TOKEN_ID;
const INIT_NONCE: u64 = 65535;

#[test]
fn test_desequip() {
    execute_for_all_slot(|slot| {
        let mut setup = utils::setup(equip_penguin::contract_obj);

        const ITEM_TO_DESEQUIP_ID: &[u8] = b"ITEM-a";
        setup.set_all_permissions_on_token(ITEM_TO_DESEQUIP_ID);
        setup.register_item(slot.clone(), ITEM_TO_DESEQUIP_ID);

        let mut b_wrapper = setup.blockchain_wrapper;
        b_wrapper.set_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            INIT_NONCE,
            &rust_biguint!(1),
            &PenguinAttributes::new(&[(
                &slot,
                Item {
                    token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_TO_DESEQUIP_ID),
                    nonce: INIT_NONCE,
                },
            )]),
        );

        let transfers = create_esdt_transfers(&[(PENGUIN_TOKEN_ID, INIT_NONCE)]);
        let _ = b_wrapper
            .execute_esdt_multi_transfer(
                &setup.first_user_address,
                &setup.cf_wrapper,
                &transfers,
                |sc| {
                    let mut managed_slots = ManagedVarArgs::<DebugApi, ItemSlot>::new();
                    managed_slots.push(slot.clone());

                    let result = sc.desequip(
                        TokenIdentifier::<DebugApi>::from_esdt_bytes(PENGUIN_TOKEN_ID),
                        INIT_NONCE,
                        BigUint::from(1u64),
                        managed_slots,
                    );

                    utils::verbose_log_if_error(&result, "".to_string());

                    assert_eq!(result, SCResult::Ok(1u64));

                    StateChange::Commit
                },
            )
            .assert_ok();

        assert_eq!(
            b_wrapper.get_esdt_balance(&setup.first_user_address, ITEM_TO_DESEQUIP_ID, INIT_NONCE),
            rust_biguint!(1)
        );

        assert_eq!(
            b_wrapper.get_esdt_balance(&setup.first_user_address, PENGUIN_TOKEN_ID, 1u64),
            rust_biguint!(1)
        );
    });
}
