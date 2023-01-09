use crate::*;
use js_sys::{Array, Error, Uint8Array};
use serde::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultJs {
    pub id: u64,
    pub distance: Option<f32>,
}

impl Drop for