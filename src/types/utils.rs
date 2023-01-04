
use crate::internals::storage_ext::*;
use std::mem;
#[cfg(nightly)]
use std::simd::*;

#[cfg(nightly)]
#[cfg(not(target_arch = "wasm32"))]
const SIMD_LANES: usize = 8;
#[cfg(nightly)]
#[cfg(target_arch = "wasm32")]
const SIMD_LANES: usize = 4;

#[cfg(nightly)]
type SimdType = Simd<f32, SIMD_LANES>;
pub const INT32_SIZE: usize = mem::size_of::<i32>();
pub const FLOAT32_SIZE: usize = mem::size_of::<f32>();

pub fn minkowski_margin(u: &[f32], v: &[f32], bias: f32) -> f32 {