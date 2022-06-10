use crate::structs::{item_slot::ItemSlot, penguin_attributes::PenguinAttributes};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("items_types")]
    fn items_slot(&self, token: &TokenIdentifier) -> SingleValueMapper<ItemSlot>;

    #[storage_mapper("penguins_identifier")]
    fn penguins_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("ipfsGateway")]
    fn ipfs_gateway(&self) -> SingleValueMapper<ManagedBuffer<Self::Api>>;

    #[storage_mapper("penguin_cid_by_attributes")]
    fn penguin_cid_by_attributes(
        &self,
        attributes: &PenguinAttributes<Self::Api>,
    ) -> SingleValueMapper<ManagedBuffer>;

    // STORAGE MODIFIERS

    #[endpoint]
    #[only_owner]
    fn set_cid(&self, attributes: &PenguinAttributes<Self::Api>, cid: ManagedBuffer<Self::Api>) {
        self.penguin_cid_by_attributes(attributes).set(cid);
    }
}
