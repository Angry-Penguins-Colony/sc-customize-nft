use crate::{constants::*, structs::image_to_render::ImageToRender};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait EquippableUrisModule: super::storage::StorageModule {
    #[storage_mapper("images_to_render")]
    fn images_to_render(&self) -> UnorderedSetMapper<ImageToRender<Self::Api>>;

    #[storage_mapper("uris_of_attributes")]
    fn uris_of_attributes(
        &self,
        attributes: &ImageToRender<Self::Api>,
    ) -> SingleValueMapper<ManagedBuffer>;

    #[endpoint(authorizeAddressToSetUris)]
    #[only_owner]
    fn authorize_address_to_set_uris(&self, address: ManagedAddress) {
        self.authorized_addresses_to_set_uris(&address).set(true);
    }

    /**
     * Endpoint of enqueue_image_to_render
     */
    #[endpoint(renderImage)]
    #[payable("EGLD")]
    fn enqueue_image_to_render(&self, attributes: &ImageToRender<Self::Api>) {
        require!(
            self.call_value().egld_value() == BigUint::from(ENQUEUE_PRICE),
            ERR_PAY_0001_EGLD
        );

        require!(
            self.uris_of_attributes(attributes).is_empty(),
            ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_ALREADY_RENDERED
        );
        require!(
            self.images_to_render().contains(attributes) == false,
            ERR_RENDER_ALREADY_IN_QUEUE
        );

        self.images_to_render().insert(attributes.clone());
    }

    #[view(getImagesToRender)]
    fn get_images_to_render(&self) -> MultiValueEncoded<ImageToRender<Self::Api>> {
        let mut o = MultiValueEncoded::new();

        for image_to_render in self.images_to_render().iter() {
            o.push(image_to_render.clone());
        }

        return o;
    }

    #[endpoint(setUriOfAttributes)]
    fn set_uri_of_attributes(
        &self,
        uri_kvp: MultiValueEncoded<MultiValue2<ImageToRender<Self::Api>, ManagedBuffer<Self::Api>>>,
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

            require!(
                self.images_to_render().contains(&attributes),
                ERR_IMAGE_NOT_IN_RENDER_QUEUE
            );

            self.uris_of_attributes(&attributes).set(uri);
            self.images_to_render().swap_remove(&attributes);
        }
    }

    #[view(getUriOf)]
    fn get_uri_of(&self, attributes: &ImageToRender<Self::Api>) -> ManagedBuffer<Self::Api> {
        let uri = self.uris_of_attributes(attributes);

        require!(
            uri.is_empty() == false,
            "There is no URI associated to the attributes {}.",
            attributes
        );

        return uri.get();
    }
}
