use crate::{constants::*, structs::equippable_attributes::EquippableAttributes};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait EquippableUrisModule: super::storage::StorageModule {
    #[storage_mapper("images_to_render")]
    fn images_to_render(
        &self,
    ) -> MapMapper<ManagedBuffer<Self::Api>, EquippableAttributes<Self::Api>>;

    #[storage_mapper("uris_of_attributes")]
    fn uris_of_attributes(
        &self,
        attributes: &EquippableAttributes<Self::Api>,
        name: &ManagedBuffer<Self::Api>,
    ) -> SingleValueMapper<ManagedBuffer>;

    #[endpoint(authorizeAddressToSetUris)]
    #[only_owner]
    fn authorize_address_to_set_uris(&self, address: ManagedAddress) {
        self.authorized_addresses_to_set_uris(&address).set(true);
    }

    /**
     * We could have used ImageToRender but we need to use the EquippableAttributes TopEncode.
     */
    #[endpoint(renderImage)]
    #[payable("EGLD")]
    fn enqueue_image_to_render(
        &self,
        attributes: &EquippableAttributes<Self::Api>,
        name: &ManagedBuffer<Self::Api>,
    ) {
        require!(
            self.call_value().egld_value() == BigUint::from(ENQUEUE_PRICE),
            ERR_PAY_0001_EGLD
        );

        require!(
            self.uris_of_attributes(&attributes, &name).is_empty(),
            ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_ALREADY_RENDERED
        );
        require!(
            self.images_to_render().contains_key(&name) == false,
            ERR_RENDER_ALREADY_IN_QUEUE
        );

        self.images_to_render()
            .insert(name.clone(), attributes.clone());
    }

    #[view(getImagesToRender)]
    fn get_images_to_render(
        &self,
    ) -> MultiValueEncoded<MultiValue2<EquippableAttributes<Self::Api>, ManagedBuffer>> {
        let mut o = MultiValueEncoded::new();

        for (name, attributes) in self.images_to_render().iter() {
            o.push(MultiValue2::from((attributes, name)));
        }

        return o;
    }

    #[endpoint(setUriOfAttributes)]
    fn set_uri_of_attributes(
        &self,
        uri_kvp: MultiValueEncoded<
            MultiValue3<EquippableAttributes<Self::Api>, ManagedBuffer, ManagedBuffer<Self::Api>>,
        >,
    ) {
        let caller = &self.blockchain().get_caller();

        require!(
            &self.blockchain().get_owner_address() == caller
                || self.authorized_addresses_to_set_uris(caller).get() == true,
            "You don't have the permission to call this endpoint."
        );

        for kvp in uri_kvp {
            let (attributes, name, uri) = kvp.into_tuple();

            require!(
                self.uris_of_attributes(&attributes, &name).is_empty(),
                ERR_CANNOT_OVERRIDE_URI_OF_ATTRIBUTE
            );

            require!(
                self.images_to_render().contains_key(&name),
                ERR_IMAGE_NOT_IN_RENDER_QUEUE
            );

            self.uris_of_attributes(&attributes, &name).set(uri);
            self.images_to_render().remove(&name);
        }
    }

    #[view(getUriOf)]
    fn get_uri_of(
        &self,
        attributes: &EquippableAttributes<Self::Api>,
        name: &ManagedBuffer<Self::Api>,
    ) -> ManagedBuffer<Self::Api> {
        let uri = self.uris_of_attributes(attributes, name);

        require!(
            uri.is_empty() == false,
            "There is no URI associated to the attributes {} for {}.",
            attributes,
            name
        );

        return uri.get();
    }
}
