elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use elrond_wasm::{
    elrond_codec::TopEncode,
    types::{ManagedBuffer, ManagedByteArray, ManagedVec},
};

use crate::structs::{item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes};

#[elrond_wasm::module]
pub trait ParsePenguin {
    fn parse_penguin_attributes(
        &self,
        penguin_id: &TokenIdentifier,
        penguin_nonce: u64,
    ) -> PenguinAttributes<Self::Api> {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &penguin_id,
                penguin_nonce,
            )
            .decode_attributes::<PenguinAttributes<Self::Api>>();

        return attributes;
    }

    fn parse_item_attributes(&self, id: &TokenIdentifier, nonce: u64) -> ItemAttributes<Self::Api> {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(&self.blockchain().get_sc_address(), &id, nonce)
            .attributes;

        return ItemAttributes {
            item_id: attributes,
        };
    }
}
