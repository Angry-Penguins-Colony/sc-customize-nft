use crate::structs::penguin_attributes::PenguinAttributes;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("penguins_identifier")]
    fn penguins_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("ipfsGateway")]
    fn ipfs_gateway(&self) -> SingleValueMapper<ManagedBuffer<Self::Api>>;

    #[storage_mapper("slot_of_items")]
    fn slot_of(&self, token: &TokenIdentifier) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("penguin_cid_by_attributes")]
    fn cid_of(&self, attributes: &PenguinAttributes<Self::Api>)
        -> SingleValueMapper<ManagedBuffer>;

    // STORAGE MODIFIERS

    #[endpoint]
    #[only_owner]
    fn set_cid(&self, attributes: &PenguinAttributes<Self::Api>, cid: ManagedBuffer<Self::Api>) {
        self.cid_of(attributes).set(cid);
    }

    #[view(hasSlot)]
    fn has_slot(&self, token: &TokenIdentifier) -> bool {
        return self.slot_of(token).is_empty() == false;
    }

    #[view(getItemType)]
    fn get_slot_of(&self, item_id: &TokenIdentifier) -> OptionalValue<ManagedBuffer> {
        if self.has_slot(item_id) {
            return OptionalValue::None;
        } else {
            return OptionalValue::Some(self.slot_of(item_id).get());
        }
    }
}
