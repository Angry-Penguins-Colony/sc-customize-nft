elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use alloc::string::ToString;
use elrond_wasm::{
    elrond_codec::TopEncode,
    types::{ManagedBuffer, ManagedByteArray, ManagedVec},
};

use crate::constants::ERR_NO_CID_URL;
use crate::structs::{item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes};

#[elrond_wasm::module]
pub trait PenguinURLBuilder: super::storage::StorageModule {
    fn build_thumbnail_url(
        &self,
        attributes: &PenguinAttributes<Self::Api>,
    ) -> ManagedBuffer<Self::Api> {
        let cid = self.thumbnail_cid_of(attributes);

        require!(cid.is_empty() == false, ERR_NO_CID_URL);

        let mut url = self.ipfs_gateway().get();
        url.append(&cid.get());

        return url;
    }
}
