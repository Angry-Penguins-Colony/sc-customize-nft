elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::structs::equippable_nft_attributes::EquippableNftAttributes;

#[elrond_wasm::module]
pub trait ParserModule {
    fn parse_equippable_attributes(
        &self,
        equippable_token_id: &TokenIdentifier, // TODO: use from storage
        equippable_nonce: u64,
    ) -> EquippableNftAttributes<Self::Api> {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &equippable_token_id,
                equippable_nonce,
            )
            .decode_attributes::<EquippableNftAttributes<Self::Api>>();

        return attributes;
    }
}
