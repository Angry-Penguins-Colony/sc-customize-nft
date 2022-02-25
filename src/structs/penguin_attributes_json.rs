#![no_std]

use elrond_wasm::{String, Vec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PenguinAttributesJSON {
    pub attributes: Vec<(String, String)>,
}
