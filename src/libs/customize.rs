use crate::{
    constants::*,
    structs::{equippable_attributes::EquippableAttributes, item::Item, token::Token},
};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait CustomizeModule:
    super::storage::StorageModule + super::equippable_uris::EquippableUrisModule
{
    #[payable("*")]
    #[endpoint(customize)]
    fn customize(&self, to_unequip_slots: MultiValueEncoded<ManagedBuffer<Self::Api>>) -> u64 {
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

        let mut attributes = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &equippable_token_id,
                equippable_nonce,
            )
            .decode_attributes::<EquippableAttributes<Self::Api>>();

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

            let token = Token::new(payment.token_identifier.clone(), payment.token_nonce);

            let printable_token_identifier = payment.token_identifier.as_managed_buffer();
            let printable_token_nonce = payment.token_nonce;
            require!(
                self.has_token(&token),
                "The item you are equipping {} {} is not registered.",
                printable_token_identifier,
                printable_token_nonce
            );

            self.equip_slot(&mut attributes, &self.map_items_tokens().get_id(&token));
        }

        return self.update_equippable(equippable_nonce, &attributes);
    }

    fn equip_slot(&self, attributes: &mut EquippableAttributes<Self::Api>, item: &Item<Self::Api>) {
        // unequip slot if any
        if attributes.is_slot_empty(&item.slot) == false {
            self.unequip_slot(attributes, &item.slot);
        }

        attributes.set_item_if_empty(&item.slot, Option::Some(item.name.clone()));
    }

    /// Empty the item at the slot provided and sent it to the caller.
    fn unequip_slot(
        &self,
        attributes: &mut EquippableAttributes<Self::Api>,
        slot: &ManagedBuffer<Self::Api>,
    ) {
        let opt_name = attributes.get_name(&slot);

        match opt_name {
            Some(name) => {
                let item = Item {
                    slot: slot.clone(),
                    name,
                };

                match self.get_token(&item) {
                    Some(token) => {
                        let item_id = token.token;
                        let item_nonce = token.nonce;

                        require!(
                            self.blockchain().get_sc_balance(
                                &EgldOrEsdtTokenIdentifier::esdt(item_id.clone()),
                                item_nonce
                            ) > 0,
                            "Can't send {}-{:x} items to the user. There is no SFT remaining.",
                            item_id,
                            item_nonce
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
                        sc_panic!(
                            "The item you are unequipping at slot {} is not registered.",
                            slot
                        );
                    }
                }
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

    fn update_equippable(
        &self,
        equippable_nonce: u64,
        attributes: &EquippableAttributes<Self::Api>,
    ) -> u64 {
        let equippable_token_id = self.equippable_token_id().get();
        let caller = self.blockchain().get_caller();

        let equippable_name = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &equippable_token_id,
                equippable_nonce,
            )
            .name;

        // mint
        let token_nonce = self.mint_equippable(&attributes, &equippable_name);

        // burn the old one
        self.send()
            .esdt_local_burn(&equippable_token_id, equippable_nonce, &BigUint::from(1u32));

        // send the new one
        self.send().direct_esdt(
            &caller,
            &equippable_token_id,
            token_nonce,
            &BigUint::from(1u32),
            &[],
        );

        return token_nonce;
    }

    fn mint_equippable(
        &self,
        attributes: &EquippableAttributes<Self::Api>,
        name: &ManagedBuffer,
    ) -> u64 {
        let mut uris = ManagedVec::new();
        let thumbnail = self.get_uri_of(&attributes, name);
        uris.push(thumbnail);

        let token_nonce = self
            .send()
            .esdt_nft_create::<EquippableAttributes<Self::Api>>(
                &self.equippable_token_id().get(),
                &BigUint::from(1u32),
                &name,
                &BigUint::zero(),
                &ManagedBuffer::new(),
                &attributes,
                &uris,
            );

        return token_nonce;
    }
}
