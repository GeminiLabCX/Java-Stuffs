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
            return Err(Error::new("item_index out of range"));
        }
        let item_vec = index.get_item_vector(item_index as u64);
        let array = Array::new();
        for v in item_vec {
            array.push(&JsValue::from_f64(v as f64));
        }
        Ok(array)
    }

    pub fn get_nearest(
        &self,
        query_vector: Array,
        n_results: u32,
        search_k: i32,
        should_include_distance: bool,
    ) -> Result<Array, E