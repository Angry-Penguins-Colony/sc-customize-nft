#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

extern crate alloc;

pub mod constants;
pub mod libs;
pub mod structs;

use alloc::string::ToString;
use elrond_wasm::{elrond_codec::TopEncode, String};
use libs::*;
use structs::{
    item::Item, item_attributes::ItemAttributes, item_slot::*,
    penguin_attributes::PenguinAttributes,
};

#[elrond_wasm::derive::contract]
pub trait Equip:
    penguin_mint::MintPenguin
    + penguin_parse::ParsePenguin
    + storage::StorageModule
    + penguin_url_builder::PenguinURLBuilder
{
    #[init]
    fn init(&self, penguins_identifier: TokenIdentifier) -> SCResult<()> {
        self.penguins_identifier().set(&penguins_identifier);
        self.ipfs_gateway().set(ManagedBuffer::new_from_bytes(
            b"https://penguins-generator.herokuapp.com/",
        ));

        return Ok(());
    }

    #[endpoint(registerItem)]
    #[only_owner]
    fn register_item(
        &self,
        item_slot: ItemSlot,
        #[var_args] items_id_to_add: MultiValueEncoded<TokenIdentifier>,
    ) -> SCResult<()> {
        require!(
            self.blockchain().get_caller() == self.blockchain().get_owner_address(),
            "Only the owner can call this method."
        );

        for item_id in items_id_to_add {
            require!(
                item_id != self.penguins_identifier().get(),
                "You cannot register a penguin as an item."
            );

            self.items_slot(&item_id.into()).set(&item_slot);
        }

        return Ok(());
    }

    #[view(getItemType)]
    fn get_item_slot(&self, item_id: &TokenIdentifier) -> OptionalValue<ItemSlot> {
        match self.items_slot(item_id).get() {
            ItemSlot::None => return OptionalValue::None,
            slot => return OptionalValue::Some(slot),
        }
    }

    #[payable("*")]
    #[endpoint(customize)]
    fn customize(
        &self,
        #[payment_multi] payments: ManagedVec<EsdtTokenPayment<Self::Api>>,
        #[var_args] to_desequip_slots: MultiValueEncoded<ItemSlot>,
    ) -> SCResult<u64> {
        self.require_penguin_roles_set()?;
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
        require!(first_payment.amount == 1, "You must sent only one penguin.");

        let mut attributes = self.parse_penguin_attributes(&penguin_id, penguin_nonce);

        // first desequip
        for slot in to_desequip_slots {
            self.desequip_slot(&mut attributes, &slot)?;
        }

        // then, equip
        let items_token = payments.iter().skip(1);
        for payment in items_token {
            require!(payment.amount == 1, "You must sent only one item.");

            let item = Item {
                token: payment.token_identifier.clone(),
                nonce: payment.token_nonce,
                name: self.get_token_name(&payment.token_identifier, payment.token_nonce)?,
            };

            self.equip_slot(&mut attributes, &item)?;
        }

        return self.update_penguin(&penguin_id, penguin_nonce, &attributes);
    }

    fn get_token_name(&self, item_id: &TokenIdentifier, nonce: u64) -> SCResult<ManagedBuffer> {
        sc_print!("token {} nonce {:x}", item_id, nonce);

        let item_name = self
            .blockchain()
            .get_esdt_token_data(&self.blockchain().get_sc_address(), item_id, nonce)
            .name;

        return Ok(item_name);
    }

    fn equip_slot(
        &self,
        attributes: &mut PenguinAttributes<Self::Api>,
        item: &Item<Self::Api>,
    ) -> SCResult<()> {
        let item_id = &item.token;
        let item_slot = self.items_slot(&item_id).get();

        require!(
            item_slot != ItemSlot::None,
            "You are trying to equip a token that is not considered as an item"
        );

        require!(
            item_id != &self.penguins_identifier().get(),
            "Cannot equip a penguin as an item."
        );

        // desequip slot if any
        if attributes.is_slot_empty(&item_slot) == false {
            self.desequip_slot(attributes, &item_slot)?;
        }

        let result = attributes.set_item(&item_slot, Option::Some(item.clone()));
        require!(
            result == Result::Ok(()),
            "Cannot set item. Maybe the item is not considered like an item."
        );

        return SCResult::Ok(());
    }

    fn require_penguin_roles_set(&self) -> SCResult<()> {
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

        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    #[only_owner]
    fn fill(
        &self,
        #[payment_token] _token: TokenIdentifier<Self::Api>,
        #[payment_nonce] _nonce: u64,
        #[payment_amount] _amount: BigUint,
    ) -> SCResult<()> {
        require!(
            self.blockchain().get_caller() == self.blockchain().get_owner_address(),
            "Only the owner can call this method."
        );

        // TODO: require! to only send registered SFT

        return Ok(());
    }

    /// Empty the item at the slot provided and sent it to the caller.
    fn desequip_slot(
        &self,
        attributes: &mut PenguinAttributes<Self::Api>,
        slot: &ItemSlot,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();

        require!(
            slot != &ItemSlot::None,
            "Slot value must be different to ItemSlot::None."
        );

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
                    self.get_item_slot(&item_id).into_option().is_some(),
                    "A item to desequip is not considered like an item. The item has maybe been removed. Please contact an administrator."
                );

                if self.blockchain().get_sc_balance(&item_id, item_nonce) <= 1 {
                    sc_panic!(
                        "No token {} with nonce {:x} on the SC. Please contact an administrator.",
                        item_id,
                        item_nonce,
                    );
                }

                self.send()
                    .direct(&caller, &item_id, item_nonce, &BigUint::from(1u32), &[]);

                let result = attributes.set_empty_slot(&slot);

                require!(result.is_err() == false, "Error while emptying slot");

                return SCResult::Ok(());
            }

            None => {
                return SCResult::Err("Slot is empty, we can't sent item to it".into());
            }
        }
    }
}
