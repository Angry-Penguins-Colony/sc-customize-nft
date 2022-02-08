#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod item;
pub mod item_attributes;
pub mod item_slot;
pub mod penguin_attributes;

use item::Item;
use item_attributes::ItemAttributes;
use item_slot::ItemSlot;
use penguin_attributes::PenguinAttributes;

#[elrond_wasm::derive::contract]
pub trait Equip {
    #[storage_mapper("items_types")]
    fn items_slot(&self, token: &TokenIdentifier) -> SingleValueMapper<ItemSlot>;

    #[storage_mapper("penguins_identifier")]
    fn penguins_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[init]
    fn init(&self, penguins_identifier: TokenIdentifier) -> SCResult<()> {
        self.penguins_identifier().set(&penguins_identifier);

        return Ok(());
    }

    #[endpoint(registerItem)]
    #[only_owner]
    fn register_item(
        &self,
        item_slot: ItemSlot,
        #[var_args] items_id_to_add: ManagedVarArgs<TokenIdentifier>,
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

            self.require_item_roles_set(&item_id)?;

            self.items_slot(&item_id.into()).set(&item_slot);
        }

        return Ok(());
    }

    #[view(getItemType)]
    fn get_item_slot(&self, item_id: &TokenIdentifier) -> OptionalResult<ItemSlot> {
        match self.items_slot(item_id).get() {
            ItemSlot::None => return OptionalResult::None,
            slot => return OptionalResult::Some(slot),
        }
    }

    #[payable("*")]
    #[endpoint]
    fn equip(
        &self,
        #[payment_multi] payments: ManagedVec<EsdtTokenPayment<Self::Api>>,
    ) -> SCResult<u64> {
        require!(payments.len() >= 2, "You must provide at least 2 tokens.");

        let first_payment = payments.get(0).unwrap();
        let penguin_id = first_payment.token_identifier;
        let penguin_nonce = first_payment.token_nonce;

        require!(
            &penguin_id == &self.penguins_identifier().get(),
            "Please provide a penguin as the first payment"
        );

        let items_token = payments
            .iter()
            .skip(1)
            .map(|payment| (payment.token_identifier, payment.token_nonce))
            .collect::<Vec<_>>();

        let mut attributes = self.parse_penguin_attributes(&penguin_id, penguin_nonce)?;

        // let's equip each item
        for (item_id, item_nonce) in items_token {
            // let (item_id, item_nonce) = item_token.into_tuple();

            require!(
                self.items_slot(&item_id).get() != ItemSlot::None,
                "You are trying to equip a token that is not considered as an item"
            );

            let item_slot_out = self.get_item_slot(&item_id);

            match item_slot_out {
                OptionalResult::None => {
                    require!(false, "An item provided is not considered like an item.")
                }
                OptionalResult::Some(item_slot) => {
                    match attributes.is_slot_empty(&item_slot) {
                        Result::Ok(false) => {
                            let result = self.sent_item_from_slot(&mut attributes, &item_slot);

                            match result {
                                SCResult::Err(err) => return SCResult::Err(err),
                                _ => (),
                            }
                        }
                        Result::Err(_) => {
                            require!(false, "Error while checking if slot is empty");
                        }
                        _ => {}
                    }

                    let result = attributes.set_item(
                        &item_slot,
                        Option::Some(Item {
                            token: item_id.clone(),
                            nonce: item_nonce,
                        }),
                    );
                    require!(
                        result == Result::Ok(()),
                        "Cannot set item. Maybe the item is not considered like an item."
                    );
                }
            }

            self.send()
                .esdt_local_burn(&item_id, item_nonce, &BigUint::from(1u32));
        }

        // update penguin
        return self.update_penguin(&penguin_id, penguin_nonce, &attributes);
    }

    #[endpoint]
    fn desequip(
        &self,
        penguin_id: &TokenIdentifier,
        penguin_nonce: u64,
        #[var_args] slots: ManagedVarArgs<ItemSlot>,
    ) -> SCResult<u64> {
        let mut attributes = self.parse_penguin_attributes(penguin_id, penguin_nonce)?;

        for slot in slots {
            let result = self.sent_item_from_slot(&mut attributes, &slot);

            match result {
                SCResult::Err(err) => return SCResult::Err(err),
                _ => (),
            }
        }

        // return items nonces
        return self.update_penguin(&penguin_id, penguin_nonce, &attributes);
    }

    fn require_item_roles_set(&self, token_id: &TokenIdentifier) -> SCResult<()> {
        let roles = self.blockchain().get_esdt_local_roles(token_id);

        require!(
            roles.has_role(&EsdtLocalRole::NftAddQuantity) == true,
            "Local add quantity role not set"
        );

        require!(
            roles.has_role(&EsdtLocalRole::NftBurn) == true,
            "Local burn role not set"
        );

        Ok(())
    }

    /// Empty the item at the slot proivided and sent it to the caller.
    fn sent_item_from_slot(
        &self,
        attributes: &mut PenguinAttributes<Self::Api>,
        slot: &ItemSlot,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();

        require!(
            attributes.is_slot_empty(&slot).unwrap() == false,
            "Cannot sent item from an empty slot"
        );

        let result = attributes.get_item(&slot);

        if let Result::Err(()) = result {
            return SCResult::Err("Error while minting and sending an item".into());
        }

        let opt_item = result.unwrap();

        match opt_item {
            Some(item) => {
                let item_id = item.token;
                let item_nonce = item.nonce;

                self.send()
                    .esdt_local_mint(&item_id, item_nonce, &BigUint::from(1u32));

                self.send()
                    .direct(&caller, &item_id, item_nonce, &BigUint::from(1u32), &[]);

                let result = attributes.empty_slot(&slot);

                require!(result.is_err() == false, "Error while emptying slot");

                return SCResult::Ok(());
            }

            None => {
                return SCResult::Err("Slot is empty, we can't sent item to it".into());
            }
        }
    }

    fn parse_penguin_attributes(
        &self,
        penguin_id: &TokenIdentifier,
        penguin_nonce: u64,
    ) -> SCResult<PenguinAttributes<Self::Api>> {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &penguin_id,
                penguin_nonce,
            )
            .decode_attributes::<PenguinAttributes<Self::Api>>();

        match attributes {
            Result::Ok(attributes) => return SCResult::Ok(attributes),
            Result::Err(_) => return SCResult::Err("Error while decoding attributes".into()),
        }
    }

    #[endpoint(mintTestPenguin)]
    #[only_owner]
    fn mint_test_penguin(&self) -> SCResult<u64> {
        let penguin_id = self.penguins_identifier().get();

        let caller = self.blockchain().get_caller();

        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new_from_bytes(b"https://www.google.com"));

        // let mut serialized_attributes = Vec::new();
        // &new_attributes.top_encode(&mut serialized_attributes)?;

        // let attributes_hash = self.crypto().sha256(&serialized_attributes);
        // let hash_buffer = ManagedBuffer::from(attributes_hash.as_bytes());

        // self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(

        let token_nonce = self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(
            &penguin_id,
            &BigUint::from(1u32),
            &ManagedBuffer::new_from_bytes(b"new penguin"),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &PenguinAttributes::empty(),
            &uris,
        );

        self.send()
            .direct(&caller, &penguin_id, token_nonce, &BigUint::from(1u32), &[]);

        return Ok(token_nonce);
    }

    #[view]
    fn empty_attributes(&self) -> PenguinAttributes<Self::Api> {
        return PenguinAttributes::empty();
    }

    fn update_penguin(
        &self,
        penguin_id: &TokenIdentifier,
        penguin_nonce: u64,
        attributes: &PenguinAttributes<Self::Api>,
    ) -> SCResult<u64> {
        let caller = self.blockchain().get_caller();

        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new_from_bytes(b"https://www.google.com"));

        // let mut serialized_attributes = Vec::new();
        // &new_attributes.top_encode(&mut serialized_attributes)?;

        // let attributes_hash = self.crypto().sha256(&serialized_attributes);
        // let hash_buffer = ManagedBuffer::from(attributes_hash.as_bytes());

        // self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(

        self.send()
            .esdt_local_burn(&penguin_id, penguin_nonce, &BigUint::from(1u32));

        let token_nonce = self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(
            &penguin_id,
            &BigUint::from(1u32),
            &ManagedBuffer::new_from_bytes(b"new penguin"),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &attributes,
            &uris,
        );

        self.send()
            .direct(&caller, &penguin_id, token_nonce, &BigUint::from(1u32), &[]);

        return Ok(token_nonce);
    }
}
