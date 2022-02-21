use crate::structs::item_slot::ItemSlot;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("items_types")]
    fn items_slot(&self, token: &TokenIdentifier) -> SingleValueMapper<ItemSlot>;

    #[storage_mapper("penguins_identifier")]
    fn penguins_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("uri")]
    fn uri(&self) -> SingleValueMapper<ManagedBuffer<Self::Api>>;
}
