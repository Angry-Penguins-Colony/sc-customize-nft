use crate::{
    constants::ERR_NO_CID_URL, structs::equippable_nft_attributes::EquippableNftAttributes, utils,
};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("equippable_token_id")]
    fn equippable_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("equippable_name_format")]
    fn equippable_name_format(&self) -> SingleValueMapper<ManagedBuffer<Self::Api>>;

    #[storage_mapper("ipfs_gateway")]
    fn ipfs_gateway(&self) -> SingleValueMapper<ManagedBuffer<Self::Api>>;

    #[storage_mapper("slot_of_items")]
    fn __slot_of(&self, token: &TokenIdentifier) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("cid_of_equippable")]
    fn __cid_of(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> SingleValueMapper<ManagedBuffer>;

    // STORAGE MODIFIERS
    // CID

    #[endpoint(setCidOf)]
    #[only_owner]
    fn set_cid_of(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
        cid: ManagedBuffer<Self::Api>,
    ) {
        self.__cid_of(attributes).set(cid);
    }

    fn get_uri_of(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> ManagedBuffer<Self::Api> {
        let cid = self.__cid_of(attributes);

        require!(cid.is_empty() == false, ERR_NO_CID_URL);

        let mut url = self.ipfs_gateway().get();
        url.append(&cid.get());

        return url;
    }

    // ===
    // SLOTS

    fn set_slot_of(&self, token: &TokenIdentifier, slot: ManagedBuffer) {
        self.__slot_of(token).set(utils::to_lowercase(&slot));
    }

    #[view(hasSlot)]
    fn has_slot(&self, token: &TokenIdentifier) -> bool {
        return self.__slot_of(token).is_empty() == false;
    }

    #[view(getItemType)]
    fn get_slot_of(&self, item_id: &TokenIdentifier) -> ManagedBuffer {
        if self.has_slot(item_id) {
            return self.__slot_of(item_id).get();
        } else {
            sc_panic!("Item {} not found.", item_id);
        }
    }
}
