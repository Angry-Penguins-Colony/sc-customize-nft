#![no_std]

use elrond_wasm::{String, Vec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PenguinAttributesJSON {
    pub attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}
