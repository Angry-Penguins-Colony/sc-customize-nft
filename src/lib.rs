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
extern crate alloc;

use alloc::string::ToString;
use elrond_wasm::String;
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

    #[storage_mapper("uri")]
    fn uri(&self) -> SingleValueMapper<ManagedBuffer<Self::Api>>;

    #[init]
    fn init(&self, penguins_identifier: TokenIdentifier) -> SCResult<()> {
        self.penguins_identifier().set(&penguins_identifier);
        self.uri().set(ManagedBuffer::new_from_bytes(
            b"https://intense-way-598.herokuapp.com/",
        ));

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
        self.require_penguin_roles_set()?;
        require!(
            payments.len() >= 2,
            "You must provide at least one penguin and one item."
        );

        let first_payment = payments.get(0);
        let penguin_id = first_payment.token_identifier;
        let penguin_nonce = first_payment.token_nonce;

        require!(
            &penguin_id == &self.penguins_identifier().get(),
            "Please provide a penguin as the first payment"
        );
        require!(first_payment.amount == 1, "You must sent only one penguin.");

        let mut attributes = self.parse_penguin_attributes(&penguin_id, penguin_nonce)?;

        // let's equip each item
        let items_token = payments.iter().skip(1);
        for payment in items_token {
            require!(payment.amount == 1, "You must sent only one item.");

            let item = Item {
                token: payment.token_identifier,
                nonce: payment.token_nonce,
            };

            self.equip_slot(&mut attributes, &item)?;
        }

        // update penguin
        return self.update_penguin(&penguin_id, penguin_nonce, &attributes);
    }

    fn equip_slot(
        &self,
        attributes: &mut PenguinAttributes<Self::Api>,
        item: &Item<Self::Api>,
    ) -> SCResult<()> {
        let item_id = &item.token;
        let item_nonce = item.nonce;

        let item_slot = self.items_slot(&item_id).get();

        require!(
            item_slot != ItemSlot::None,
            "You are trying to equip a token that is not considered as an item"
        );

        require!(
            item_id != &self.penguins_identifier().get(),
            "Cannot equip a penguin as an item."
        );

        self.require_item_roles_set(&item_id)?;

        // desequip slot if any
        if attributes.is_slot_empty(&item_slot) == false {
            self.desequip_slot(attributes, &item_slot)?;
        }

        let result = attributes.set_item(
            &item_slot,
            Option::Some(Item {
                token: item_id.clone(),
                nonce: item_nonce.clone(),
            }),
        );
        require!(
            result == Result::Ok(()),
            "Cannot set item. Maybe the item is not considered like an item."
        );

        self.send()
            .esdt_local_burn(&item_id, item_nonce, &BigUint::from(1u32));

        return SCResult::Ok(());
    }

    #[payable("*")]
    #[endpoint]
    fn desequip(
        &self,
        #[payment_token] penguin_id: TokenIdentifier,
        #[payment_nonce] penguin_nonce: u64,
        #[payment_amount] amount: BigUint<Self::Api>,
        #[var_args] slots: ManagedVarArgs<ItemSlot>,
    ) -> SCResult<u64> {
        require!(amount == 1, "You can desequip only one penguin at a time.");
        require!(
            penguin_id == self.penguins_identifier().get(),
            "Sent token is not a penguin."
        );

        let mut attributes = self.parse_penguin_attributes(&penguin_id, penguin_nonce)?;

        for slot in slots {
            self.desequip_slot(&mut attributes, &slot)?;
        }

        // return items nonces
        return self.update_penguin(&penguin_id, penguin_nonce, &attributes);
    }

    fn require_item_roles_set(&self, token_id: &TokenIdentifier) -> SCResult<()> {
        let roles = self.blockchain().get_esdt_local_roles(token_id);

        require!(
            roles.has_role(&EsdtLocalRole::NftAddQuantity) == true,
            "Local add quantity role not set for an item"
        );

        require!(
            roles.has_role(&EsdtLocalRole::NftBurn) == true,
            "Local burn role not set for an item"
        );

        Ok(())
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

        // TODO: require! that the future balance will be equals to 1
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
                self.require_item_roles_set(&item_id)?;

                if self.blockchain().get_sc_balance(&item_id, item_nonce) == 0 {
                    sc_panic!(
                        "To mint the token {} with nonce {:x}, the SC must owns at least one.",
                        item_id,
                        item_nonce,
                    );
                }

                self.send()
                    .esdt_local_mint(&item_id, item_nonce, &BigUint::from(1u32));

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

    #[view]
    fn get_items_attributes(&self) -> ItemAttributes<Self::Api> {
        return ItemAttributes {
            item_id: ManagedBuffer::new_from_bytes(b"test"),
        };
    }

    #[endpoint(mintTestPenguin)]
    #[only_owner]
    fn mint_test_penguin(&self) -> SCResult<u64> {
        let penguin_id = self.penguins_identifier().get();

        let caller = self.blockchain().get_caller();

        let mut uris = ManagedVec::new();
        uris.push(self.build_url(&PenguinAttributes::empty())?);

        // let mut serialized_attributes = Vec::new();
        // &new_attributes.top_encode(&mut serialized_attributes)?;

        // let attributes_hash = self.crypto().sha256(&serialized_attributes);
        // let hash_buffer = ManagedBuffer::from(attributes_hash.as_bytes());

        // self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(

        let token_nonce = self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(
            &penguin_id,
            &BigUint::from(1u32),
            &self.build_penguin_name_buffer(),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &PenguinAttributes::empty(),
            &uris,
        );

        self.send()
            .direct(&caller, &penguin_id, token_nonce, &BigUint::from(1u32), &[]);

        return Ok(token_nonce);
    }

    fn build_penguin_name_buffer(&self) -> ManagedBuffer {
        let penguin_id = self.penguins_identifier().get();

        let index = self
            .blockchain()
            .get_current_esdt_nft_nonce(&self.blockchain().get_sc_address(), &penguin_id)
            + 1;

        let mut full_token_name = ManagedBuffer::new();
        let token_name_from_storage = ManagedBuffer::new_from_bytes(b"Penguin");
        let hash_sign = ManagedBuffer::new_from_bytes(" #".as_bytes());
        let token_index = ManagedBuffer::new_from_bytes(index.to_string().as_bytes());

        full_token_name.append(&token_name_from_storage);
        full_token_name.append(&hash_sign);
        full_token_name.append(&token_index);

        return full_token_name;
    }

    #[view]
    fn get_empty_attributes(&self) -> PenguinAttributes<Self::Api> {
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
        uris.push(self.build_url(&attributes)?);

        // let mut serialized_attributes = Vec::new();
        // &new_attributes.top_encode(&mut serialized_attributes)?;

        // let attributes_hash = self.crypto().sha256(&serialized_attributes);
        // let hash_buffer = ManagedBuffer::from(attributes_hash.as_bytes());

        let name = self.get_penguin_name(penguin_nonce);

        self.send()
            .esdt_local_burn(&penguin_id, penguin_nonce, &BigUint::from(1u32));

        let token_nonce = self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(
            &penguin_id,
            &BigUint::from(1u32),
            &name,
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &attributes,
            &uris,
        );

        self.send()
            .direct(&caller, &penguin_id, token_nonce, &BigUint::from(1u32), &[]);

        return Ok(token_nonce);
    }

    fn build_url(
        &self,
        attributes: &PenguinAttributes<Self::Api>,
    ) -> SCResult<ManagedBuffer<Self::Api>> {
        if attributes.get_fill_count() == 0 {
            return SCResult::Ok(self.get_full_unequiped_penguin_uri());
        }

        let mut expected = ManagedBuffer::new();
        expected.append(&self.uri().get());

        let mut is_first_item = true;

        for slot in ItemSlot::VALUES.iter() {
            if let Some(item) = attributes.get_item(slot) {
                let token_data = self.parse_item_attributes(&item.token, item.nonce)?;

                let slot_type = token_data.item_id;
                let slot_id = slot.to_bytes::<Self::Api>();

                if is_first_item == false {
                    expected.append_bytes(b"+");
                }

                expected.append(&ManagedBuffer::new_from_bytes(slot_id));
                expected.append_bytes(b"_");
                expected.append(&slot_type);

                is_first_item = false;
            }
        }

        expected.append_bytes(b"/image.png");

        return SCResult::Ok(expected);
    }

    fn parse_item_attributes(
        &self,
        id: &TokenIdentifier,
        nonce: u64,
    ) -> SCResult<ItemAttributes<Self::Api>> {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(&self.blockchain().get_sc_address(), &id, nonce)
            .decode_attributes::<ItemAttributes<Self::Api>>();

        match attributes {
            Result::Ok(attributes) => return SCResult::Ok(attributes),
            Result::Err(err) => {
                sc_panic!(
                    "Error while decoding item {}{}{} attributes: {}",
                    id,
                    "-",
                    nonce.to_string(),
                    err.message_str(),
                );
            }
        }
    }

    fn get_penguin_name(&self, penguin_nonce: u64) -> ManagedBuffer<Self::Api> {
        let nft_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &self.penguins_identifier().get(),
            penguin_nonce,
        );

        return nft_data.name;
    }

    fn get_full_unequiped_penguin_uri(&self) -> ManagedBuffer<Self::Api> {
        let mut uri = ManagedBuffer::new();

        uri.append(&self.uri().get());
        uri.append_bytes(b"empty/image.png");

        return uri;
    }
}
