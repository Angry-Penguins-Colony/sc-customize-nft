#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Debug, PartialEq, Clone)]
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

    pub fn to_bytes<M: ManagedTypeApi>(&self) -> &[u8] {
        match self {
            Self::Hat => return b"hat",
            Self::Background => return b"background",
            Self::Skin => return b"skin",
            Self::Chain => return b"chain",
            Self::Beak => return b"beak",
            Self::Weapon => return b"weapon",
            Self::Clothes => return b"clothes",
            Self::Eye => return b"eyes",
            Self::None => return b"none",
        }
    }
}
