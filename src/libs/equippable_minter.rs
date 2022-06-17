elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use elrond_wasm::{
    elrond_codec::TopEncode,
    types::{ManagedBuffer, ManagedByteArray, ManagedVec},
};

use crate::{
    constants::EQUIPPABLE_NAME_FORMAT_NUMBER,
    structs::equippable_nft_attributes::EquippableNftAttributes, utils::ManagedBufferUtils,
};
use crate::{constants::ERR_NO_CID_URL, utils};

#[elrond_wasm::module]
pub trait MintEquippableModule:
    super::storage::StorageModule + super::parser::ParserModule
{
    /// Burn old eqquipable, and mint a new one.
    fn update_equippable(
        &self,
        equippable_token_id: &TokenIdentifier, // TODO: equippable_token_id is registered somewhere in storage, can we remove this arg ?
        equippable_nonce: u64,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> u64 {
        let caller = self.blockchain().get_caller();

        // mint
        let token_nonce =
            self.mint_equippable(attributes, &self.get_equippable_name(equippable_nonce));

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

    fn get_equippable_name(&self, nonce: u64) -> ManagedBuffer<Self::Api> {
        let nft_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &self.equippable_token_id().get(),
            nonce,
        );

        return nft_data.name;
    }

    fn mint_equippable(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
        name: &ManagedBuffer,
    ) -> u64 {
        let mut uris = ManagedVec::new();
        let thumbnail = self.get_uri_of(&attributes);
        uris.push(thumbnail);

        let token_nonce = self
            .send()
            .esdt_nft_create::<EquippableNftAttributes<Self::Api>>(
                &self.equippable_token_id().get(),
                &BigUint::from(1u32),
                &name,
                &BigUint::zero(),
                &self.calculate_hash(&attributes),
                &attributes,
                &uris,
            );

        return token_nonce;
    }

    fn calculate_hash(&self, _attributes: &EquippableNftAttributes<Self::Api>) -> ManagedBuffer {
        // we disabled hash calculating for now
        return ManagedBuffer::new();
    }

    fn get_next_equippable_name(&self) -> ManagedBuffer {
        let index = self.blockchain().get_current_esdt_nft_nonce(
            &self.blockchain().get_sc_address(),
            &self.equippable_token_id().get(),
        ) + 1;

        let token_index = utils::u64_to_ascii(&index);
        let token_name = self
            .equippable_name_format()
            .get()
            .replace(EQUIPPABLE_NAME_FORMAT_NUMBER, &token_index);

        return token_name;
    }
}
