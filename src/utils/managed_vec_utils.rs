use core::ops::Deref;

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{ManagedBuffer, ManagedVec, ManagedVecItem},
};

use crate::utils::managed_buffer_utils::ManagedBufferUtils;

pub trait EqUtils<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    /// Check if two unordered vec are equals
    fn eq_unorder(&self, other: &Self) -> bool;
}

impl<M, T> EqUtils<M, T> for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + PartialEq + Clone,
{
    fn eq_unorder(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut other_copy: ManagedVec<M, T> = other.clone();

        for item in self.into_iter() {
            let opt_item_index = other_copy.find(&item);

            if opt_item_index.is_none() {
                return false;
            }

            other_copy.remove(opt_item_index.unwrap());
        }

        return other_copy.len() == 0;
    }
}

pub trait SortUtils<M>
where
    M: ManagedTypeApi,
{
    fn sort_alphabetically(&self) -> Self;
}
impl<M> SortUtils<M> for ManagedVec<M, ManagedBuffer<M>>
where
    M: ManagedTypeApi,
{
    fn sort_alphabetically(&self) -> Self {
        let mut remaining_items = self.clone();
        let mut output = Self::new();

        while remaining_items.len() > 0 {
            let mut smallest_item = remaining_items.get(0);

            for item in remaining_items.iter() {
                if item.compare(&smallest_item).is_le() {
                    smallest_item = item;
                }
            }

            output.push(smallest_item.deref().clone());
            remaining_items.remove(remaining_items.find(&smallest_item).unwrap());
        }

        return output;
    }
}
