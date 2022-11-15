use crate::{
    structs::{
        equippable_attributes::{
            panic_if_name_contains_unsupported_characters,
            panic_if_slot_contains_unsupported_characters,
        },
        item::Item,
        token::Token,
    },
    utils::bidimapper_utils::ContainsUtils,
};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("equippable_token_id")]
    fn equippable_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("royalties_overrided")]
    fn royalties_overrided(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("mapper_items_token")]
    fn map_items_tokens(&self) -> BiDiMapper<Self::Api, Item<Self::Api>, Token<Self::Api>>;

    #[storage_mapper("authorized_addresses_to_set_uris")]
    fn authorized_addresses_to_set_uris(&self) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

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

    fn insert_or_replace_item_token(&self, item: Item<Self::Api>, token: Token<Self::Api>) {
        panic_if_name_contains_unsupported_characters(&Option::Some(item.name.clone()));
        panic_if_slot_contains_unsupported_characters(&item.slot);

        if self.has_item(&item) {
            self.map_items_tokens().remove_by_id(&item);
        }

        if self.has_token(&token) {
            self.map_items_tokens().remove_by_value(&token);
        }

        let _ = self.map_items_tokens().insert(item, token);
    }
}
