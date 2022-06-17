#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![feature(generic_associated_types)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod constants;
pub mod libs;
pub mod structs;
pub mod utils;

use elrond_wasm::elrond_codec::TopEncode;
use libs::*;
use structs::{
    equippable_nft_attributes::EquippableNftAttributes, item::Item, item_attributes::ItemAttributes,
};

use crate::constants::{
    ERR_BURN_ROLE_NOT_SET_FOR_EQUIPPABLE, ERR_CANNOT_EQUIP_EQUIPPABLE,
    ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM, ERR_CANNOT_UNEQUIP_EMPTY_SLOT,
    ERR_CREATE_ROLE_NOT_SET_FOR_EQUIPPABLE, ERR_FIRST_PAYMENT_IS_EQUIPPABLE,
    ERR_ITEM_TO_UNEQUIP_HAS_NO_SLOT, ERR_MORE_THAN_ONE_EQUIPPABLE_RECEIVED,
    ERR_MORE_THAN_ONE_ITEM_RECEIVED, ERR_NEED_EQUIPPABLE, ERR_NEED_ONE_ITEM_OR_UNEQUIP_SLOT,
    ERR_NOT_OWNER,
};

#[elrond_wasm::derive::contract]
pub trait Equip:
    equippable_minter::MintEquippableModule + parser::ParserModule + storage::StorageModule
{
    #[init]
    fn init(&self, equippable_token_id: TokenIdentifier, gateway: ManagedBuffer) {
        self.equippable_token_id().set(&equippable_token_id);

        // if the user forgot the backslash, we add it silently
        let valid_gateway = utils::append_trailing_character_if_missing(&gateway, b'/');
        self.ipfs_gateway().set(valid_gateway);
    }

    #[endpoint(registerItem)]
    #[only_owner]
    fn register_item(
        &self,
        item_slot: ManagedBuffer,
        items_id_to_add: MultiValueEncoded<TokenIdentifier>,
    ) {
        require!(
            self.blockchain().get_caller() == self.blockchain().get_owner_address(),
            ERR_NOT_OWNER
        );

        for item_id in items_id_to_add {
            require!(
                item_id != self.equippable_token_id().get(),
                ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM
            );

            self.set_slot_of(&item_id, item_slot.clone());
        }
    }

    #[payable("*")]
    #[endpoint(customize)]
    fn customize(
        &self,
        #[payment_multi] payments: ManagedVec<EsdtTokenPayment<Self::Api>>,
        to_unequip_slots: MultiValueEncoded<ManagedBuffer>,
    ) -> u64 {
        self.require_equippable_collection_roles_set();
        require!(payments.len() >= 1, ERR_NEED_EQUIPPABLE);
        require!(
            payments.len() >= 2 || to_unequip_slots.len() >= 1,
            ERR_NEED_ONE_ITEM_OR_UNEQUIP_SLOT
        );

        let first_payment = payments.get(0);
        let equippable_token_id = first_payment.token_identifier;
        let equippable_nonce = first_payment.token_nonce;

        require!(
            &equippable_token_id == &self.equippable_token_id().get(),
            ERR_FIRST_PAYMENT_IS_EQUIPPABLE
        );

        require!(
            first_payment.amount == BigUint::from(1u64),
            ERR_MORE_THAN_ONE_EQUIPPABLE_RECEIVED
        );

        let mut attributes =
            self.parse_equippable_attributes(&equippable_token_id, equippable_nonce);

        // first unequip
        for slot in to_unequip_slots {
            self.unequip_slot(&mut attributes, &slot);
        }

        // then, equip
        let items_token = payments.iter().skip(1);
        for payment in items_token {
            require!(
                payment.amount == BigUint::from(1u64),
                ERR_MORE_THAN_ONE_ITEM_RECEIVED
            );

            let item = Item {
                token: payment.token_identifier.clone(),
                nonce: payment.token_nonce,
                name: self.get_token_name(&payment.token_identifier, payment.token_nonce),
            };

            self.equip_slot(&mut attributes, &item);
        }

        return self.update_equippable(&equippable_token_id, equippable_nonce, &attributes);
    }

    fn get_token_name(&self, item_id: &TokenIdentifier, nonce: u64) -> ManagedBuffer {
        let item_name = self
            .blockchain()
            .get_esdt_token_data(&self.blockchain().get_sc_address(), item_id, nonce)
            .name;

        return item_name;
    }

    fn equip_slot(
        &self,
        attributes: &mut EquippableNftAttributes<Self::Api>,
        item: &Item<Self::Api>,
    ) {
        let item_id = &item.token;

        require!(
            self.has_slot(&item_id) == true,
            "Trying to equip {} but is not considered as an item", // TODO: extract to constant
            item_id
        );

        require!(
            item_id != &self.equippable_token_id().get(),
            ERR_CANNOT_EQUIP_EQUIPPABLE
        );

        let item_slot = self.get_slot_of(&item_id);

        // unequip slot if any
        if attributes.is_slot_empty(&item_slot) == false {
            self.unequip_slot(attributes, &item_slot);
        }

        attributes.set_item(&item_slot, Option::Some(item.clone()));
    }

    /// Make sure that the smart contract can create and burn the equippable.
    fn require_equippable_collection_roles_set(&self) {
        let roles = self
            .blockchain()
            .get_esdt_local_roles(&self.equippable_token_id().get());

        require!(
            roles.has_role(&EsdtLocalRole::NftCreate) == true,
            ERR_CREATE_ROLE_NOT_SET_FOR_EQUIPPABLE
        );

        require!(
            roles.has_role(&EsdtLocalRole::NftBurn) == true,
            ERR_BURN_ROLE_NOT_SET_FOR_EQUIPPABLE
        );
    }

    #[payable("*")]
    #[endpoint]
    #[only_owner]
    fn fill(
        &self,
        #[payment_token] _token: EgldOrEsdtTokenIdentifier<Self::Api>,
        #[payment_nonce] _nonce: u64,
        #[payment_amount] _amount: BigUint,
    ) {
        require!(
            self.blockchain().get_caller() == self.blockchain().get_owner_address(),
            ERR_NOT_OWNER
        );

        // TODO: require! to only send registered SFT
    }

    /// Empty the item at the slot provided and sent it to the caller.
    fn unequip_slot(
        &self,
        attributes: &mut EquippableNftAttributes<Self::Api>,
        slot: &ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();

        let opt_item = attributes.get_item(&slot);

        match opt_item {
            Some(item) => {
                let item_id = item.token;
                let item_nonce = item.nonce;

                require!(
                    self.has_slot(&item_id) == true,
                    ERR_ITEM_TO_UNEQUIP_HAS_NO_SLOT
                );

                if self.blockchain().get_sc_balance(
                    &EgldOrEsdtTokenIdentifier::esdt(item_id.clone()),
                    item_nonce,
                ) <= BigUint::from(1u64)
                {
                    sc_panic!(
                        "No token {} with nonce {:x} on the smart contract wallet. Please contact an administrator.", // TODO: extract to constant
                        item_id,
                        item_nonce,
                    );
                }

                self.send()
                    .direct_esdt(&caller, &item_id, item_nonce, &BigUint::from(1u32), &[]);

                attributes.empty_slot(&slot);
            }

            None => {
                sc_panic!(ERR_CANNOT_UNEQUIP_EMPTY_SLOT);
            }
        }
    }
}
