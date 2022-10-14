use crate::{
    constants::{
        ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_CID_ALREADY_RENDERER, ERR_RENDER_ALREADY_IN_QUEUE,
    },
    structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item},
    utils::{managed_buffer_utils::ManagedBufferUtils, vec_mapper_utils::VecMapperUtils},
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

    #[storage_mapper("items_token")]
    fn token_of(
        &self,
        item: &Item<Self::Api>,
    ) -> SingleValueMapper<(TokenIdentifier<Self::Api>, u64)>;

    #[storage_mapper("permissions")]
    fn __permissions_set_cid_of(
        &self,
        address: &ManagedAddress<Self::Api>,
    ) -> SingleValueMapper<bool>;

    #[storage_mapper("slot_of_items")]
    fn __slot_of(&self, token: &TokenIdentifier) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("equippable_name_format")]
    fn __images_to_render(&self) -> VecMapper<EquippableNftAttributes<Self::Api>>;

    #[storage_mapper("cid_of_equippable")]
    fn __cid_of(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> SingleValueMapper<ManagedBuffer>;

    // STORAGE MODIFIERS
    // CID

    #[endpoint(addPermissionSetCid)]
    #[only_owner]
    fn add_permission_set_cid(&self, address: ManagedAddress) {
        self.__permissions_set_cid_of(&address).set(true);
    }

    #[endpoint(setCidOf)]
    fn set_cid_of(
        &self,
        cid_kvp: MultiValueEncoded<
            MultiValue2<EquippableNftAttributes<Self::Api>, ManagedBuffer<Self::Api>>,
        >,
    ) {
        let caller = &self.blockchain().get_caller();

        require!(
            &self.blockchain().get_owner_address() == caller
                || self.__permissions_set_cid_of(caller).get() == true,
            "You don't have the permission to call this endpoint."
        );

        for kvp in cid_kvp {
            let (attributes, cid) = kvp.into_tuple();

            self.__cid_of(&attributes).set(cid);
            self.__images_to_render().remove_item(&attributes);
        }
    }

    #[view(getCidOf)]
    fn get_cid_of(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> ManagedBuffer<Self::Api> {
        return self.__cid_of(attributes).get();
    }

    fn cid_of_exist(&self, attributes: &EquippableNftAttributes<Self::Api>) -> bool {
        return !self.__cid_of(attributes).is_empty();
    }

    fn get_uri_of(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> ManagedBuffer<Self::Api> {
        let cid = self.__cid_of(attributes);

        require!(
            cid.is_empty() == false,
            "There is no CID associated to the attributes {}.",
            attributes
        );

        let mut url = self.ipfs_gateway().get();
        url.append(&cid.get());

        return url;
    }

    // ===
    // SLOTS

    fn set_slot_of(&self, token: &TokenIdentifier, slot: ManagedBuffer) {
        self.__slot_of(token).set(&slot.to_lowercase());
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
            sc_panic!("No slot found for {}.", item_id);
        }
    }

    // ===
    // IMAGES
    fn enqueue_image_to_render(&self, attributes: &EquippableNftAttributes<Self::Api>) -> usize {
        require!(
            self.cid_of_exist(attributes) == false,
            ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_CID_ALREADY_RENDERER
        );
        require!(
            self.__images_to_render().has_item(attributes) == false,
            ERR_RENDER_ALREADY_IN_QUEUE
        );

        self.__images_to_render().push(attributes)
    }

    #[view(getImagesToRender)]
    fn get_images_to_render(&self) -> MultiValueEncoded<EquippableNftAttributes<Self::Api>> {
        let mut o = MultiValueEncoded::new();

        for attributes in self.__images_to_render().iter() {
            o.push(attributes.clone());
        }

        return o;
    }
}
