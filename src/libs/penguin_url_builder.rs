elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use alloc::string::ToString;
use elrond_wasm::{
    elrond_codec::TopEncode,
    types::{ManagedBuffer, ManagedByteArray, ManagedVec, SCResult},
};

use crate::constants::ERR_NO_CID_URL;
use crate::structs::{item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes};

#[elrond_wasm::module]
pub trait PenguinURLBuilder: super::storage::StorageModule {
    fn build_url(&self, attributes: &PenguinAttributes<Self::Api>) -> ManagedBuffer<Self::Api> {
        let cid = self.penguin_cid_by_attributes(attributes);

        require!(cid.is_empty() == false, ERR_NO_CID_URL);

        let mut url = self.ipfs_gateway().get();
        url.append(&cid.get());

        return url;
    }
}
