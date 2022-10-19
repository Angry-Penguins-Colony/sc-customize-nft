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
pub struct Token<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub nonce: u64,
}

impl<M: ManagedTypeApi> Token<M> {
    pub fn new(token: TokenIdentifier<M>, nonce: u64) -> Self {
        Self { token, nonce }
    }
}

impl<M: ManagedTypeApi> Default for Token<M> {
    fn default() -> Self {
        Self {
            token: TokenIdentifier::from_esdt_bytes(b""),
            nonce: Default::default(),
        }
    }
}
