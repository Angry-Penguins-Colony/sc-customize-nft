elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use alloc::string::ToString;
use elrond_wasm::{
    elrond_codec::TopEncode,
    types::{ManagedBuffer, ManagedByteArray, ManagedVec, SCResult},
};

use crate::constants::ERR_NO_CID_URL;
use crate::structs::{item_slot::ItemSlot, penguin_attributes::PenguinAttributes, utils};

#[elrond_wasm::module]
pub trait MintPenguin:
    super::storage::StorageModule
    + super::penguin_parse::ParsePenguin
    + super::penguin_url_builder::PenguinURLBuilder
{
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
        let thumbnail = self.build_thumbnail_url(&attributes)?;
        uris.push(thumbnail);

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

    fn calculate_hash(
        &self,
        _attributes: &PenguinAttributes<Self::Api>,
    ) -> SCResult<ManagedBuffer> {
        // we disabled hash calculating for now
        return SCResult::Ok(ManagedBuffer::new());
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
