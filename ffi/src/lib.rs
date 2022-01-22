
#![allow(clippy::missing_safety_doc)]

#[macro_use]
mod macros;

pub use annoy_rs;
use annoy_rs::*;

use libc::c_char;
use std::error::Error;
use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::slice;

ffi_fn! {
    fn load_annoy_index(path: *const c_char, dimension: i32, index_type: u8) -> *const AnnoyIndex {
        let result = load_annoy_index_inner(path, dimension, index_type);
        match result {
            Ok(ptr) => ptr,
            Err(_e) => ptr::null(),
        }
    }
}

fn load_annoy_index_inner(
    path: *const c_char,
    dimension: i32,
    index_type: u8,
) -> Result<*const AnnoyIndex, Box<dyn Error>> {
    let c_str_path = unsafe { CStr::from_ptr(path) };
    let ru_path = c_str_path.to_str()?;
    let ru_index_type: IndexType = unsafe { mem::transmute(index_type) };
    let index = AnnoyIndex::load(dimension as usize, ru_path, ru_index_type)?;
    Ok(Box::into_raw(Box::new(index)))
}

ffi_fn! {