use elrond_wasm::{
    api::StorageMapperApi,
    elrond_codec::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    storage::{mappers::BiDiMapper, StorageKey},
    storage_get, storage_get_len,
    types::ManagedType,
};

const NULL_ENTRY: usize = 0;
const ITEM_INDEX: &[u8] = b".index";

pub trait ContainsUtils<SA, K, V> {
    fn contains_id(&self, id: &K, base_key: &[u8]) -> bool;
    fn contains_value(&self, value: &V, base_key: &[u8]) -> bool;
}

impl<SA, K, V> ContainsUtils<SA, K, V> for BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    fn contains_id(&self, id: &K, base_key: &[u8]) -> bool {
        let mut key = StorageKey::<SA>::new(base_key);
        key.append_bytes(b"_id");
        key.append_bytes(ITEM_INDEX);
        key.append_item(id);

        storage_get::<SA, usize>(key.as_ref()) != NULL_ENTRY
    }

    fn contains_value(&self, value: &V, base_key: &[u8]) -> bool {
        let mut key = StorageKey::<SA>::new(base_key);
        key.append_bytes(b"_value");
        key.append_bytes(ITEM_INDEX);
        key.append_item(value);

        storage_get::<SA, usize>(key.as_ref()) != NULL_ENTRY
    }
}
