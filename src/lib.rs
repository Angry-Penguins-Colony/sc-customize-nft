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

#[elrond_wasm::derive::contract]
pub trait Equip:
    penguin_mint::MintPenguin
    + penguin_parse::ParsePenguin
    + storage::StorageModule
    + penguin_url_builder::PenguinURLBuilder
{
    #[init]
    fn init(&self, penguins_identifier: TokenIdentifier, gateway: ManagedBuffer) {
        self.penguins_identifier().set(&penguins_identifier);

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
            "Only the owner can call this method."
        );

        for item_id in items_id_to_add {
            require!(
                item_id != self.penguins_identifier().get(),
                "You cannot register a penguin as an item."
            );

            self.set_slot_of(&item_id, item_slot.clone());
        }
    }

    #[payable("*")]
    #[endpoint(customize)]
    fn customize(
        &self,
        #[payment_multi] payments: ManagedVec<EsdtTokenPayment<Self::Api>>,
        to_desequip_slots: MultiValueEncoded<ManagedBuffer>,
    ) -> u64 {
        self.require_penguin_roles_set();
        require!(
            payments.len() >= 1,
            "You must provide at least one penguin."
        );
        require!(
            payments.len() >= 2 || to_desequip_slots.len() >= 1,
            "You must either provide at least one penguin and one item OR provide a slot to desequip."
        );

        let first_payment = payments.get(0);
        let penguin_id = first_payment.token_identifier;
        let penguin_nonce = first_payment.token_nonce;

        require!(
            &penguin_id == &self.penguins_identifier().get(),
            "Please provide a penguin as the first payment"
        );

        require!(
            first_payment.amount == BigUint::from(1u64),
            "You must sent only one penguin."
        );

        let mut attributes = self.parse_penguin_attributes(&penguin_id, penguin_nonce);

        // first desequip
        for slot in to_desequip_slots {
            self.desequip_slot(&mut attributes, &slot);
        }

        // then, equip
        let items_token = payments.iter().skip(1);
        for payment in items_token {
            require!(
                payment.amount == BigUint::from(1u64),
                "You must sent only one item."
            );

            let item = Item {
                token: payment.token_identifier.clone(),
                nonce: payment.token_nonce,
                name: self.get_token_name(&payment.token_identifier, payment.token_nonce),
            };

            self.equip_slot(&mut attributes, &item);
        }

        return self.update_penguin(&penguin_id, penguin_nonce, &attributes);
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
            "Trying to equip {} but is not considered as an item",
            item_id
        );

        require!(
            item_id != &self.penguins_identifier().get(),
            "Cannot equip a penguin as an item."
        );

        let item_slot = self.get_slot_of(&item_id);

        sc_print!("Equipping {} in slot {}", item_id, item_slot);

        // desequip slot if any
        if attributes.is_slot_empty(&item_slot) == false {
            self.desequip_slot(attributes, &item_slot);
        }

        attributes.set_item(&item_slot, Option::Some(item.clone()));
    }

    fn require_penguin_roles_set(&self) {
        let penguin_id = self.penguins_identifier().get();
        let roles = self.blockchain().get_esdt_local_roles(&penguin_id);

        require!(
            roles.has_role(&EsdtLocalRole::NftCreate) == true,
            "Local create role not set for penguin"
        );

        require!(
            roles.has_role(&EsdtLocalRole::NftBurn) == true,
            "Local burn role not set  for penguin"
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
            "Only the owner can call this method."
        );

        // TODO: require! to only send registered SFT
    }

    /// Empty the item at the slot provided and sent it to the caller.
    fn desequip_slot(
        &self,
        attributes: &mut EquippableNftAttributes<Self::Api>,
        slot: &ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();

        require!(
            attributes.is_slot_empty(&slot) == false,
            "Cannot sent item from an empty slot"
        );

        let opt_item = attributes.get_item(&slot);

        match opt_item {
            Some(item) => {
                let item_id = item.token;
                let item_nonce = item.nonce;

                require!(
                    self.has_slot(&item_id) == true,
                    "A item to desequip is not considered like an item. The item has maybe been removed. Please contact an administrator."
                );

                if self.blockchain().get_sc_balance(
                    &EgldOrEsdtTokenIdentifier::esdt(item_id.clone()),
                    item_nonce,
                ) <= BigUint::from(1u64)
                {
                    sc_panic!(
                        "No token {} with nonce {:x} on the SC. Please contact an administrator.",
                        item_id,
                        item_nonce,
                    );
                }

                self.send()
                    .direct_esdt(&caller, &item_id, item_nonce, &BigUint::from(1u32), &[]);

                attributes.empty_slot(&slot);
            }

            None => {
                sc_panic!("Slot is empty, we can't sent item to it");
            }
        }
    }
}
