use crate::{
    structs::{item::Item, token::Token},
    utils::bidimapper_utils::ContainsUtils,
};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("equippable_token_id")]
    fn equippable_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("mapper_items_token")]
    fn map_items_tokens(&self) -> BiDiMapper<Self::Api, Item<Self::Api>, Token<Self::Api>>;

    #[storage_mapper("authorized_addresses_to_set_uris")]
    fn authorized_addresses_to_set_uris(
        &self,
        address: &ManagedAddress<Self::Api>,
    ) -> SingleValueMapper<bool>;

    fn has_item(&self, item: &Item<Self::Api>) -> bool {
        return self
            .map_items_tokens()
            .contains_id(item, b"mapper_items_token");
    }

    fn has_token(&self, token: &Token<Self::Api>) -> bool {
        return self
            .map_items_tokens()
            .contains_value(token, b"mapper_items_token");
    }

    fn get_item(&self, token: &Token<Self::Api>) -> Option<Item<Self::Api>> {
        if self.has_token(token) == false {
            return None;
        } else {
            return Some(self.map_items_tokens().get_id(token));
        }
    }

    fn get_token(&self, item: &Item<Self::Api>) -> Option<Token<Self::Api>> {
        if self.has_item(item) == false {
            return None;
        } else {
            return Some(self.map_items_tokens().get_value(item));
        }
    }
}
