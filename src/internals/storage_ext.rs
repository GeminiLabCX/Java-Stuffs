use std::mem;
use std::slice;

pub(crate) trait StorageExtensions {
    fn read_i32(&self, idx: usize) -> i32;
    fn read_f32(&self, idx: usize) -> f32;