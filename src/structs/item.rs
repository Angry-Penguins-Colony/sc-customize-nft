elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(
    ManagedVecItem,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    PartialEq,
    TypeAbi,
    Clone,
    Debug,
)]
pub struct Item<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub slot: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> Default for Item<M> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            slot: Default::default(),
        }
    }
}
