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

impl Drop for SearchResultJs {
    fn drop(&mut self) {}
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct AnnoyIndexJs {
    pub dimension: usize,
    pub size: usize,

    index_ptr: *const AnnoyIndex,
}

impl Drop for AnnoyIndexJs {
    fn drop(&mut self) {
        self.free()
    }
}

#[wasm_bindgen]
impl AnnoyIndexJs {
    pub fn free(&self) {
        unsafe {
            drop(Box::from_raw(self.index_ptr as *mut AnnoyIndex));
        }
    }

    pub fn get_item_vector(&self, item_index: u32) -> Result<Array, Error> {
        let index = unsafe { &*self.index_ptr };
        if (item_index as usize) >= index.size {
  