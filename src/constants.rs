pub const ERR_NO_CID_URL: &str = "";
pub const ERR_INIT_MISSING_NUMBER_FORMAT: &str = "The name format require {number} somewhere.";
pub const ERR_NOT_OWNER: &str = "Only the owner can call this method.";
pub const ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM: &str =
    "You cannot register an equippable NFT as an item.";
pub const ERR_NEED_EQUIPPABLE: &str = "You must send the equippable NFT to equip.";
pub const ERR_NEED_ONE_ITEM_OR_UNEQUIP_SLOT: &str =
    "You must either send an item to equip OR set a slot to unequip.";
pub const ERR_FIRST_PAYMENT_IS_EQUIPPABLE: &str =
    "The first token sent must be the equippable NFT.";
pub const ERR_MORE_THAN_ONE_EQUIPPABLE_RECEIVED: &str =
    "Sending more than one equippable NFT is not supported.";
pub const ERR_MORE_THAN_ONE_ITEM_RECEIVED: &str = "Sending more than one item is not supported.";
pub const ERR_CANNOT_EQUIP_EQUIPPABLE: &str =
    "Cannot equip an equippable NFT over another equippable NFT.";
pub const ERR_CREATE_ROLE_NOT_SET_FOR_EQUIPPABLE: &str =
    "This smart contract lacks the create role in the collection of equipable NFTs.";
pub const ERR_BURN_ROLE_NOT_SET_FOR_EQUIPPABLE: &str =
    "This smart contract lacks the burn role in the collection of equipable NFTs.";
pub const ERR_CANNOT_UNEQUIP_EMPTY_SLOT: &str = "Cannot unequip an empty slot";
pub const ERR_ITEM_TO_UNEQUIP_HAS_NO_SLOT: &str =
    "Item to unequip has no slot. Please, contact an admin.";

pub const EQUIPPABLE_NAME_FORMAT_NUMBER: &[u8] = b"{number}";
