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
    Beak,
    Weapon,
    Clothes,
    Eye,
}

impl ItemSlot {
    pub const VALUES: [ItemSlot; 7] = [
        Self::Hat,
        Self::Background,
        Self::Skin,
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
            Self::Beak => return b"beak",
            Self::Weapon => return b"weapon",
            Self::Clothes => return b"clothes",
            Self::Eye => return b"eyes",
            Self::None => return b"none",
        }
    }
}
