pub(crate) mod annoy_index_impl;
pub(crate) mod node;
pub(crate) mod utils;

pub mod serving;
pub use serving::*;
use std::{
    fmt::{Display, Formatter, Result},
    ops::Index,
};

use crate::internals::storage_ext::StorageExtensions;

#[derive(Debug, Clone)]
pub struct AnnoyIndexSearchResult {
    pub count: usize,
    pub is_distance_included: bool,
    pub id_list: Vec<u64>,
    pub distance_list: Vec<f32>,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
// #[wasm_bindgen::prelude::wasm_bindgen]
pub enum IndexType {
    Angular = 0,
    Euclidean = 1,
    Manhattan = 2,
    Hamming = 3,
    Dot = 4,
}

impl Display for IndexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let r = format!("{self:?}").to_lowercase();
        f.write_str(&r)
    }
}

#[derive(Debug)]
pub(crate) enum Storage {
    #[cfg(not(target_arch = "wasm32"))]
    Mmap(Box<memmap2::Mmap>),
    Buffer(Vec<u8>),
}

impl StorageExtensions for Storage {
    fn read_i32(&self, idx: usize) -> i32 {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::Mmap(mmap) => mmap.read_i32(idx),
            Self::Buffer(buffer) => buffer.read_i32(idx),
        }
    }

    fn read_f32(&self, idx: usize) -> f32 {
        match self {
            #[cfg(not(t