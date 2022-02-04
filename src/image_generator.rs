#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use crate::penguin_attributes::PenguinAttributes;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait ImageGenerator {
    fn generate_image(&self, attributes: &PenguinAttributes<Self::Api>) {
        // order each slot by layer
        // get image from item
        // merge them
        // return the image
    }
}
