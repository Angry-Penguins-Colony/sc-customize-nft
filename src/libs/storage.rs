use crate::{
    constants::ERR_CANNOT_OVERRIDE_URI_OF_ATTRIBUTE,
    structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item, token::Token},
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

    #[storage_mapper("images_to_render")]
    fn images_to_render(&self) -> UnorderedSetMapper<EquippableNftAttributes<Self::Api>>;

    #[storage_mapper("uris_of_attributes")]
    fn uris_of_attributes(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> SingleValueMapper<ManagedBuffer>;

    fn has_item(&self, item: &Item<Self::Api>) -> bool {
        return self.map_items_tokens().iter().any(|i| &i.0 == item);
    }

    fn has_token(&self, token: &Token<Self::Api>) -> bool {
        return self.map_items_tokens().iter().any(|i| &i.1 == token);
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
    // STORAGE MODIFIERS

    #[endpoint(authorizeAddressToSetUris)]
    #[only_owner]
    fn authorize_address_to_set_uris(&self, address: ManagedAddress) {
        self.authorized_addresses_to_set_uris(&address).set(true);
    }

    #[endpoint(setUriOfAttributes)]
    fn set_uri_of_attributes(
        &self,
        uri_kvp: MultiValueEncoded<
            MultiValue2<EquippableNftAttributes<Self::Api>, ManagedBuffer<Self::Api>>,
        >,
    ) {
        let caller = &self.blockchain().get_caller();

        require!(
            &self.blockchain().get_owner_address() == caller
                || self.authorized_addresses_to_set_uris(caller).get() == true,
            "You don't have the permission to call this endpoint."
        );

        for kvp in uri_kvp {
            let (attributes, uri) = kvp.into_tuple();

            require!(
                self.uris_of_attributes(&attributes).is_empty(),
                ERR_CANNOT_OVERRIDE_URI_OF_ATTRIBUTE
            );

            self.uris_of_attributes(&attributes).set(uri);
            self.images_to_render().swap_remove(&attributes);
        }
    }

    #[view(getUriOf)]
    fn get_uri_of(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> ManagedBuffer<Self::Api> {
        let uri = self.uris_of_attributes(attributes);

        require!(
            uri.is_empty() == false,
            "There is no URI associated to the attributes {}.",
            attributes
        );

        return uri.get();
    }

    // ===
    // IMAGES
    #[view(getImagesToRender)]
    fn get_images_to_render(&self) -> MultiValueEncoded<EquippableNftAttributes<Self::Api>> {
        let mut o = MultiValueEncoded::new();

        for attributes in self.images_to_render().iter() {
            o.push(attributes.clone());
        }

        return o;
    }
}
