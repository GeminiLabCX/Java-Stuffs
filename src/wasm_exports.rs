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
    ) -> Result<Array, Error> {
        let index = unsafe { &*self.index_ptr };
        if query_vector.length() as usize != index.dimension {
            return Err(Error::new(&format!(
                "Wrong input dimension, {} expected, {} provided.",
                index.dimension,
                query_vector.length()
            )));
        }
        let mut vec = Vec::with_capacity(index.dimension);
        for i in 0..(index.dimension as i32) {
            let v = query_vector.at(i);
            if let Some(v) = v.as_f64() {
                vec.push(v as f32);
            } else {
                return Err(Error::new("Input array should be of number type."));
            }
        }
        let result = index.get_nearest(
            vec.as_slice(),
            n_results as usize,
            search_k,
            should_include_distance,
        );
        convert_result(result)
    }

    pub fn get_nearest_to_item(
        &self,
        item_index: u32,
        n_results: u32,
        search_k: i32,
        should_include_distance: bool,
    ) -> Result<Array, Error> {
        let index = unsafe { &*self.index_ptr };
        if (item_index as usize) >= index.size {
            return Err(Error::new("item_index