use std::mem;
use std::slice;

pub(crate) trait StorageExtensions {
    fn read_i32(&self, idx: usize) -> i32;
    fn read_f32(&self, idx: usize) -> f32;
    fn read_slice<T: Sized>(&self, idx: usize, len: usize) -> &[T];
}

#[cfg(not(target_arch = "wasm32"))]
impl StorageExtensions for memmap2::Mmap {
    fn read_i32(&self, idx: usize) -> i32 {
        // let ptr: *const i32 = unsafe { mem::transmute(&self[idx]) };
        let ptr: 