#![no_std]
#![no_main]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod constants;
pub mod libs;
pub mod structs;
pub mod utils;

use libs::*;
use structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item, slot::Slot};

use crate::constants::*;

#[elrond_wasm::derive::contract]
pub trait Equip:
    equippable_minter::MintEquippableModule + parser::ParserModule + storage::StorageModule
{
    #[init]
    fn init(&self, equippable_token_id: TokenIdentifier) {
        self.equippable_token_id().set(&equippable_token_id);
    }

    #[endpoint(registerItem)]
    #[only_owner]
    fn register_item(
        &self,
        slot: Slot<Self::Api>,
        name: ManagedBuffer<Self::Api>,
        token_id: TokenIdentifier<Self::Api>,
        token_nonce: u64,
    ) {
        require!(
            token_id != self.equippable_token_id().get(),
            ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM
        );

        let item = &Item { name, slot };

        let storage_item = self.get_item_from_token(&token_id, token_nonce);
        let storage_token = self.get_token_from_item(item);

        require!(storage_item.is_empty(), ERR_CANNOT_OVERRIDE_REGISTERED_ITEM);
        require!(
            storage_token.is_empty(),
            ERR_CANNOT_OVERRIDE_REGISTERED_ITEM
        );

        storage_item.set(item);
        storage_token.set((token_id, token_nonce));
    }

    #[payable("*")]
    #[endpoint]
    #[only_owner]
    fn fill(&self) {
        let payment = self.call_value().single_esdt();

        require!(
            self.get_item_from_token(&payment.token_identifier, payment.token_nonce)
                .is_empty()
                == false,
            ERR_CANNOT_FILL_UNREGISTERED_ITEM
        )
    }

    /**
     * Endpoint of enqueue_image_to_render
     */
    #[endpoint(renderImage)]
    #[payable("EGLD")]
    fn add_image_to_render(&self, attributes: &EquippableNftAttributes<Self::Api>) {
        require!(
            self.call_value().egld_value() == BigUint::from(ENQUEUE_PRICE),
            ERR_PAY_0001_EGLD
        );

        self.enqueue_image_to_render(&attributes);
    }

    #[payable("*")]
    #[endpoint(customize)]
    fn customize(&self, to_unequip_slots: MultiValueEncoded<Slot<Self::Api>>) -> u64 {
        let payments = self.call_value().all_esdt_transfers();

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

            let opt_item = self.get_item_from_token(&payment.token_identifier, payment.token_nonce);

            if opt_item.is_empty() {
                sc_print!(
                    "Not registered {}-{}",
                    payment.token_identifier,
                    payment.token_nonce
                );
            }

            require!(
                opt_item.is_empty() == false,
                ERR_CANNOT_EQUIP_UNREGISTED_ITEM
            );

            self.equip_slot(&mut attributes, &opt_item.get());
        }

        return self.update_equippable(&equippable_token_id, equippable_nonce, &attributes);
    }

    #[only_owner]
    #[endpoint(claim)]
    fn claim(&self) {
        let balance = self
            .blockchain()
            .get_balance(&self.blockchain().get_sc_address());

        self.send()
            .direct_egld(&self.blockchain().get_owner_address(), &balance, b"");
    }

    fn equip_slot(
        &self,
        attributes: &mut EquippableNftAttributes<Self::Api>,
        item: &Item<Self::Api>,
    ) {
        // unequip slot if any
        if attributes.is_slot_empty(&item.slot) == false {
            self.unequip_slot(attributes, &item.slot);
        }

        attributes.set_item_if_empty(&item.slot, Option::Some(item.name.clone()));
    }

    /// Empty the item at the slot provided and sent it to the caller.
    fn unequip_slot(
        &self,
        attributes: &mut EquippableNftAttributes<Self::Api>,
        slot: &Slot<Self::Api>,
    ) {
        let opt_item = attributes.get_item(&slot);

        match opt_item {
            Some(item) => {
                let storage_token = self.get_token_from_item(&item);

                if storage_token.is_empty() {
                    sc_print!("Not registered {}-{}", item.slot.get(), item.name);
                }

                require!(!storage_token.is_empty(), ERR_CANNOT_EQUIP_UNREGISTED_ITEM,);

                let (item_id, item_nonce) = storage_token.get();

                require!(
                    self.blockchain().get_sc_balance(
                        &EgldOrEsdtTokenIdentifier::esdt(item_id.clone()),
                        item_nonce
                    ) > 0,
                    "Can't send unequipped items to the user. There is no SFT remaining."
                );

                self.send().direct_esdt(
                    &self.blockchain().get_caller(),
                    &item_id,
                    item_nonce,
                    &BigUint::from(1u32),
                    &[],
                );

                attributes.empty_slot(&slot);
            }

            None => {
                sc_panic!(ERR_CANNOT_UNEQUIP_EMPTY_SLOT);
            }
        }
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
}
