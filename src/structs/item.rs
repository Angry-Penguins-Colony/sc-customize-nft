use super::slot::Slot;

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
    pub slot: Slot<M>,
}
