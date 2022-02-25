elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use alloc::string::ToString;
use elrond_wasm::{
    elrond_codec::TopEncode,
    types::{ManagedBuffer, ManagedByteArray, ManagedVec, SCResult},
};

use crate::structs::{item_slot::ItemSlot, penguin_attributes::PenguinAttributes};

#[elrond_wasm::module]
pub trait MintPenguin: super::storage::StorageModule + super::penguin_parse::ParsePenguin {
    #[endpoint(mintTestPenguin)]
    #[only_owner]
    fn mint_test_penguin(&self) -> SCResult<u64> {
        let penguin_id = self.penguins_identifier().get();
        let caller = self.blockchain().get_caller();

        let token_nonce =
            self.mint_penguin(&PenguinAttributes::empty(), &self.get_next_penguin_name())?;

        self.send()
            .direct(&caller, &penguin_id, token_nonce, &BigUint::from(1u32), &[]);

        return SCResult::Ok(token_nonce);
    }

    fn update_penguin(
        &self,
        penguin_id: &TokenIdentifier,
        penguin_nonce: u64,
        attributes: &PenguinAttributes<Self::Api>,
    ) -> SCResult<u64> {
        let caller = self.blockchain().get_caller();

        // mint
        let token_nonce = self.mint_penguin(attributes, &self.get_penguin_name(penguin_nonce))?;

        // burn the old one
        self.send()
            .esdt_local_burn(&penguin_id, penguin_nonce, &BigUint::from(1u32));

        // send the new one
        self.send()
            .direct(&caller, &penguin_id, token_nonce, &BigUint::from(1u32), &[]);

        return SCResult::Ok(token_nonce);
    }

    fn get_penguin_name(&self, penguin_nonce: u64) -> ManagedBuffer<Self::Api> {
        let nft_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &self.penguins_identifier().get(),
            penguin_nonce,
        );

        return nft_data.name;
    }

    fn mint_penguin(
        &self,
        attributes: &PenguinAttributes<Self::Api>,
        name: &ManagedBuffer,
    ) -> SCResult<u64> {
        let penguin_id = self.penguins_identifier().get();

        let mut uris = ManagedVec::new();
        uris.push(self.build_url(&attributes)?);

        let token_nonce = self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(
            &penguin_id,
            &BigUint::from(1u32),
            &name,
            &BigUint::zero(),
            &self.calculate_hash(&attributes)?,
            &attributes,
            &uris,
        );

        return SCResult::Ok(token_nonce);
    }

    fn calculate_hash(&self, attributes: &PenguinAttributes<Self::Api>) -> SCResult<ManagedBuffer> {
        let mut serialized_attributes = ManagedBuffer::new();
        if let core::result::Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }

        let attributes_hash: &H256 = &self
            .crypto()
            .sha256_legacy(serialized_attributes.to_boxed_bytes().as_slice());

        let managed_buffer = ManagedBuffer::new_from_bytes(attributes_hash.as_bytes());

        return SCResult::Ok(managed_buffer);
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

    fn get_full_unequiped_penguin_uri(&self) -> ManagedBuffer<Self::Api> {
        let mut uri = ManagedBuffer::new();

        uri.append(&self.uri().get());
        uri.append_bytes(b"empty/image.png");

        return uri;
    }

    fn get_next_penguin_name(&self) -> ManagedBuffer {
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
}
