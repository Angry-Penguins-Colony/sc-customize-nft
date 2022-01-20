use elrond_wasm::types::{ManagedBuffer, ManagedVarArgs, SCResult};
use elrond_wasm_debug::testing_framework::*;
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::item_attributes::ItemAttributes;
use equip_penguin::item_slot::ItemSlot;
use equip_penguin::penguins_attributes::PenguinAttributes;
use equip_penguin::*;
use utils::create_esdt_transfers;

mod utils;

const PENGUIN_TOKEN_ID: &[u8] = utils::PENGUIN_TOKEN_ID;
const HAT_TOKEN_ID: &[u8] = utils::HAT_TOKEN_ID;
const INIT_NONCE: u64 = 65535;

#[test]
fn test_desequip() {
    let mut setup = utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    let transfers = create_esdt_transfers(&[(PENGUIN_TOKEN_ID, INIT_NONCE)]);

    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &PenguinAttributes {
            hat: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
                INIT_NONCE,
            ),
        },
    );

    b_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        &transfers,
        |sc| {
            let mut managed_slots = ManagedVarArgs::<DebugApi, ItemSlot>::new();
            managed_slots.push(ItemSlot::Hat);

            let result = sc.desequip(
                &TokenIdentifier::<DebugApi>::from_esdt_bytes(PENGUIN_TOKEN_ID),
                INIT_NONCE,
                managed_slots,
            );

            assert_eq!(result, SCResult::Ok(1u64));

            StateChange::Commit
        },
    );

    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        1u64,
        &rust_biguint!(1),
        &PenguinAttributes {
            hat: (
                TokenIdentifier::<DebugApi>::from(ManagedBuffer::new()),
                0u64,
            ),
        },
    );

    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        HAT_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &ItemAttributes {},
    )
}
