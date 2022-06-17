elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use elrond_wasm::{
    elrond_codec::TopEncode,
    types::{ManagedBuffer, ManagedByteArray, ManagedVec},
};

use crate::constants::ERR_NO_CID_URL;
use crate::structs::{
    equippable_nft_attributes::EquippableNftAttributes, item_attributes::ItemAttributes,
};

#[elrond_wasm::module]
pub trait URLBuilder: super::storage::StorageModule {
    fn build_thumbnail_url(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> ManagedBuffer<Self::Api> {
        let cid = self.cid_of(attributes);

        require!(cid.is_empty() == false, ERR_NO_CID_URL);

        let mut url = self.ipfs_gateway().get();
        url.append(&cid.get());

        return url;
    }
}
