use std::ops::Deref;

use customize_nft::utils::managed_vec_utils::SortUtils;
use elrond_wasm::types::{ManagedBuffer, ManagedVec};
use elrond_wasm_debug::{managed_buffer, testing_framework::BlockchainStateWrapper, DebugApi};

#[test]
fn do_nothing_if_empty() {
    BlockchainStateWrapper::new().execute_in_managed_environment(|| {
        let sorted = ManagedVec::<DebugApi, ManagedBuffer<DebugApi>>::new().sort_alphabetically();
        assert_eq!(sorted.len(), 0);
    });
}

#[test]
fn do_nothing_if_one_elment() {
    BlockchainStateWrapper::new().execute_in_managed_environment(|| {
        let sorted = ManagedVec::<DebugApi, ManagedBuffer<DebugApi>>::from_single_item(
            managed_buffer!(b"hello"),
        )
        .sort_alphabetically();
        assert_eq!(sorted.len(), 1);
    });
}

#[test]
fn should_sort() {
    BlockchainStateWrapper::new().execute_in_managed_environment(|| {
        let sorted = ManagedVec::<DebugApi, ManagedBuffer<DebugApi>>::from(vec![
            managed_buffer!(b"b"),
            managed_buffer!(b"a"),
        ])
        .sort_alphabetically();

        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted.get(0).deref(), &managed_buffer!(b"a"));
        assert_eq!(sorted.get(1).deref(), &managed_buffer!(b"b"));
    });
}

#[test]
fn should_sort_number() {
    BlockchainStateWrapper::new().execute_in_managed_environment(|| {
        let sorted = ManagedVec::<DebugApi, ManagedBuffer<DebugApi>>::from(vec![
            managed_buffer!(b"b"),
            managed_buffer!(b"1"),
            managed_buffer!(b"a"),
        ])
        .sort_alphabetically();

        assert_eq!(sorted.len(), 3);
        assert_eq!(
            sorted,
            ManagedVec::<DebugApi, ManagedBuffer<DebugApi>>::from(vec![
                managed_buffer!(b"1"),
                managed_buffer!(b"a"),
                managed_buffer!(b"b"),
            ])
        )
    });
}

#[test]
fn should_sort_while_ignore_case() {
    BlockchainStateWrapper::new().execute_in_managed_environment(|| {
        let sorted = ManagedVec::<DebugApi, ManagedBuffer<DebugApi>>::from(vec![
            managed_buffer!(b"b"),
            managed_buffer!(b"1"),
            managed_buffer!(b"A"),
            managed_buffer!(b"a"),
        ])
        .sort_alphabetically();

        assert_eq!(sorted.len(), 4);
        assert_eq!(
            sorted,
            ManagedVec::<DebugApi, ManagedBuffer<DebugApi>>::from(vec![
                managed_buffer!(b"1"),
                managed_buffer!(b"A"),
                managed_buffer!(b"a"),
                managed_buffer!(b"b"),
            ])
        );
    })
}
