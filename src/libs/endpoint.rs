use crate::{
    constants::{ENQUEUE_PRICE, ERR_PAY_0001_EGLD},
    structs::equippable_nft_attributes::EquippableNftAttributes,
};

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait EndpointsModule: super::storage::StorageModule {
    /**
     * Endpoint of enqueue_image_to_render
     */
    #[endpoint(renderImage)]
    #[payable("EGLD")]
    fn render_image(&self, attributes: &EquippableNftAttributes<Self::Api>) {
        sc_print!("egld {}", self.call_value().egld_value());
        require!(
            self.call_value().egld_value() == BigUint::from(ENQUEUE_PRICE),
            ERR_PAY_0001_EGLD
        );

        self.enqueue_image_to_render(&attributes);
    }
}
