use elrond_wasm::{
    api::StorageMapperApi,
    elrond_codec::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    storage::{
        mappers::{BiDiMapper, StorageMapper, UnorderedSetMapper},
        StorageKey,
    },
};

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

        UnorderedSetMapper::new(key).contains(id)
    }

    fn contains_value(&self, value: &V, base_key: &[u8]) -> bool {
        let mut key = StorageKey::<SA>::new(base_key);
        key.append_bytes(b"_value");

        UnorderedSetMapper::new(key).contains(value)
    }
}
