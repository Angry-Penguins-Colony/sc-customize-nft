use super::equippable_attributes::EquippableAttributes;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone, Debug)]
pub struct ImageToRender<M: ManagedTypeApi> {
    pub attributes: EquippableAttributes<M>,
    pub name: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ImageToRender<M> {
    pub fn to_multi_value_encoded(&self) -> MultiValue2<EquippableAttributes<M>, ManagedBuffer<M>> {
        return MultiValue2::from((self.attributes.clone(), self.name.clone()));
    }
}
