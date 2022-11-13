
use super::utils::*;
use super::{AnnoyIndex, IndexType};
use crate::internals::storage_ext::*;
use crate::types::node::*;
use crate::Storage;
use std::error::Error;
use std::vec::Vec;

impl AnnoyIndex {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load(
        dimension: usize,
        index_file_path: &str,
        index_type: IndexType,
    ) -> Result<AnnoyIndex, Box<dyn Error>> {
        let file = std::fs::File::open(index_file_path)?;
        let file_metadata = std::fs::metadata(index_file_path)?;
        let file_size = file_metadata.len() as i64;
        let storage = Storage::Mmap(Box::new(unsafe { memmap2::MmapOptions::new().map(&file)? }));
        Self::load_inner(dimension, file_size, index_type, storage)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_into_mem(
        dimension: usize,
        index_file_path: &str,
        index_type: IndexType,
    ) -> Result<AnnoyIndex, Box<dyn Error>> {
        let buffer = std::fs::read(index_file_path)?;
        let size = buffer.len() as i64;
        let storage = Storage::Buffer(buffer);
        Self::load_inner(dimension, size, index_type, storage)
    }

    pub fn load_from_buffer(
        buffer: Vec<u8>,
        dimension: usize,
        index_type: IndexType,
    ) -> Result<AnnoyIndex, Box<dyn Error>> {
        let size = buffer.len() as i64;
        let storage = Storage::Buffer(buffer);
        Self::load_inner(dimension, size, index_type, storage)
    }

    fn load_inner(
        dimension: usize,
        index_size: i64,
        index_type: IndexType,
        storage: Storage,
    ) -> Result<AnnoyIndex, Box<dyn Error>> {
        let (offset_before_children, node_header_size, max_descendants): (usize, usize, usize) =
            match index_type {
                IndexType::Angular => (4, NodeHeaderAngular::header_size(), dimension + 2),