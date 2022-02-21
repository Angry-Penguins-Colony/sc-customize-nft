elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use alloc::string::ToString;
use elrond_wasm::{
    elrond_codec::TopEncode,
    types::{ManagedBuffer, ManagedByteArray, ManagedVec, SCResult},
};

use crate::structs::{item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes};

#[elrond_wasm::module]
pub trait ParsePenguin {
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
}
