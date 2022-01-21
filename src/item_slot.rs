elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Debug, PartialEq)]
pub enum ItemSlot {
    None,
    Hat,
    Background,
    Skin,
    Chain,
    Beak,
    Weapon,
    Clothes,
    Eye,
}

impl ItemSlot {
    pub const VALUES: [ItemSlot; 8] = [
        Self::Hat,
        Self::Background,
        Self::Skin,
        Self::Chain,
        Self::Beak,
        Self::Weapon,
        Self::Clothes,
        Self::Eye,
    ];
}
