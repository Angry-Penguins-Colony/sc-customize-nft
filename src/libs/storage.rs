use crate::{
    constants::{
        ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_ALREADY_RENDERED, ERR_CANNOT_OVERRIDE_URI_OF_ATTRIBUTE,
        ERR_RENDER_ALREADY_IN_QUEUE,
    },
    structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item, slot::Slot},
};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("equippable_token_id")]
    fn equippable_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("items_token")]
    fn token_of_item(
        &self,
        item: &Item<Self::Api>,
    ) -> SingleValueMapper<(TokenIdentifier<Self::Api>, u64)>;

    #[storage_mapper("whitelist_set_uris_of_attributes_endpoint")]
    fn whitelist_set_uris_of_attributes_endpoint(
        &self,
        address: &ManagedAddress<Self::Api>,
    ) -> SingleValueMapper<bool>;

    #[storage_mapper("slot_of_items")]
    fn slot_of_item(&self, token: &TokenIdentifier) -> SingleValueMapper<Slot<Self::Api>>;

    #[storage_mapper("__images_to_render")]
    fn images_to_render(&self) -> UnorderedSetMapper<EquippableNftAttributes<Self::Api>>;

    #[storage_mapper("uris_of_attributes")]
    fn uris_of_attributes(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> SingleValueMapper<ManagedBuffer>;

    // STORAGE MODIFIERS

    #[endpoint(addPermissionToSetUris)]
    #[only_owner]
    fn add_permission_to_set_uris_attributes(&self, address: ManagedAddress) {
        self.whitelist_set_uris_of_attributes_endpoint(&address)
            .set(true);
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
                || self.whitelist_set_uris_of_attributes_endpoint(caller).get() == true,
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

    fn do_uri_exists_for(&self, attributes: &EquippableNftAttributes<Self::Api>) -> bool {
        return !self.uris_of_attributes(attributes).is_empty();
    }

    // ===
    // SLOTS

    fn set_slot_of(&self, token: &TokenIdentifier, slot: Slot<Self::Api>) {
        self.slot_of_item(token).set(&slot);
    }

    #[view(hasSlot)]
    fn has_slot(&self, token: &TokenIdentifier) -> bool {
        return self.slot_of_item(token).is_empty() == false;
    }

    #[view(getItemType)]
    fn get_slot_of(&self, item_id: &TokenIdentifier) -> Slot<Self::Api> {
        if self.has_slot(item_id) {
            return self.slot_of_item(item_id).get();
        } else {
            sc_panic!("No slot found for {}.", item_id);
        }
    }

    // ===
    // IMAGES
    fn enqueue_image_to_render(&self, attributes: &EquippableNftAttributes<Self::Api>) {
        require!(
            self.do_uri_exists_for(attributes) == false,
            ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_ALREADY_RENDERED
        );
        require!(
            self.images_to_render().contains(attributes) == false,
            ERR_RENDER_ALREADY_IN_QUEUE
        );

        self.images_to_render().insert(attributes.clone());
    }

    #[view(getImagesToRender)]
    fn get_images_to_render(&self) -> MultiValueEncoded<EquippableNftAttributes<Self::Api>> {
        let mut o = MultiValueEncoded::new();

        for attributes in self.images_to_render().iter() {
            o.push(attributes.clone());
        }

        return o;
    }
}
