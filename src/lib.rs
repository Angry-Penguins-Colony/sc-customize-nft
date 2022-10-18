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

use crate::{constants::*, utils::managed_buffer_utils::ManagedBufferUtils};

#[elrond_wasm::derive::contract]
pub trait Equip:
    equippable_minter::MintEquippableModule + parser::ParserModule + storage::StorageModule
{
    #[init]
    fn init(&self, equippable_token_id: TokenIdentifier, gateway: ManagedBuffer) {
        require!(gateway.get_last_char() == b'/', ERR_GATEWAY_NEEDS_SLASH);

        self.equippable_token_id().set(&equippable_token_id);
        self.ipfs_gateway().set(gateway);
    }

    #[endpoint(registerItem)]
    #[only_owner]
    fn register_item(
        &self,
        item_slot: Slot<Self::Api>,
        items_id_to_add: MultiValueEncoded<TokenIdentifier>,
    ) {
        for item_id in items_id_to_add {
            require!(
                item_id != self.equippable_token_id().get(),
                ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM
            );

            self.set_slot_of(&item_id, item_slot.clone());
        }
    }

    #[payable("*")]
    #[endpoint]
    #[only_owner]
    fn fill(&self) {
        // TODO: require! to only send registered SFT

        let payment = self.call_value().single_esdt();
        let token_id = payment.token_identifier;
        let token_nonce = payment.token_nonce;

        // TODO: extract to Item.fromId()
        let item_name = self
            .blockchain()
            .get_esdt_token_data(&self.blockchain().get_sc_address(), &token_id, token_nonce)
            .name;

        let item = &Item {
            name: item_name.clone(),
            slot: self.get_slot_of(&token_id),
        };

        require!(
            self.token_of_item(item).is_empty(),
            "The item with name {} is already registered. Please, use another name.",
            item_name
        );
        self.token_of_item(item)
            .set((token_id.clone(), token_nonce));
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

            let item = Item {
                name: self.get_token_name(&payment.token_identifier, payment.token_nonce),
                slot: self.get_slot_of(&payment.token_identifier),
            };

            self.equip_slot(&mut attributes, &item);
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
        let n = item.clone().name;

        require!(
            self.token_of_item(&item).is_empty() == false,
            "Trying to equip '{}' but is not considered as an item", // TODO: extract to constant
            n
        );

        let (item_id, _) = self.token_of_item(&item).get();

        require!(
            self.has_slot(&item_id) == true,
            "Trying to equip {} but is not considered as an item", // TODO: extract to constant
            item_id
        );

        require!(
            item_id != self.equippable_token_id().get(),
            ERR_CANNOT_EQUIP_EQUIPPABLE
        );

        let item_slot = self.get_slot_of(&item_id);

        // unequip slot if any
        if attributes.is_slot_empty(&item_slot) == false {
            self.unequip_slot(attributes, &item_slot);
        }

        attributes.set_item_if_empty(&item_slot, Option::Some(item.name.clone()));
    }

    /// Empty the item at the slot provided and sent it to the caller.
    fn unequip_slot(
        &self,
        attributes: &mut EquippableNftAttributes<Self::Api>,
        slot: &Slot<Self::Api>,
    ) {
        let caller = self.blockchain().get_caller();

        let opt_item = attributes.get_item(&slot);

        match opt_item {
            Some(item) => {
                let n = item.clone().name;

                require!(
                    self.token_of_item(&item).is_empty() == false,
                    "{} token is empty, while it should be filled.",
                    n
                );

                let (item_id, item_nonce) = self.token_of_item(&item).get();

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
