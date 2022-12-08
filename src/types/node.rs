use crate::{IndexType, Storage};
use std::mem;

pub(crate) struct Node {
    pub offset: usize,
    pub header: NodeHeader,
}

impl Node {
    pub fn new_with_id(
        id: usize,
        node_size: usize,
        index_type: &IndexType,
        storage: &Storage,
    ) -> Node {
        let offset = id * node_size;
        let header = NodeHeader::new(offset, index_type, storage);
        Node { offset, header }
    }
}

#[repr(C)]
pub(crate) enum NodeHeader {
  