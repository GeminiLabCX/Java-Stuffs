use crate::{IndexType, Storage};
use std::mem;

pub(crate) struct Node {
    pub offset: usize,
    pub header: NodeHeader,
}

impl Node {
    pub fn new_with_id(
        id: usize,
        