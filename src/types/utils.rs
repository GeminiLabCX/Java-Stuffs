
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
    bias + dot_product(u, v)
}

/*
template<typename S, typename T, typename Distance, typename Random, class ThreadedBuildPolicy>
  class AnnoyIndex : public AnnoyIndexInterface<S, T,
  AnnoyIndex<int32_t, uint64_t, Hamming, Kiss64Random, AnnoyIndexThreadedBuildPolicy> _index;
  static inline bool margin(const Node<S, T>* n, const T* y, int f) {
    static const size_t n_bits = sizeof(T) * 8;
    T chunk = n->v[0] / n_bits;
    return (y[chunk] & (static_cast<T>(1) << (n_bits - 1 - (n->v[0] % n_bits)))) != 0;
  }
*/
// pub fn hamming_margin(u: &[f32], v: &[f32], bias: f32) -> f32 {
//     return bias + dot_product(u, v);
// }

// #[inline(never)]
pub fn dot_product(u: &[f32], v: &[f32]) -> f32 {
    cfg_if! {
        if #[cfg(nightly)] {
            dot_product_simd(u, v)
        } else {
            dot_product_no_simd(u, v)
        }
    }
}

#[cfg(any(test, not(nightly)))]
pub fn dot_product_no_simd(u: &[f32], v: &[f32]) -> f32 {
    u.iter().zip(v.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(nightly)]
pub fn dot_product_simd(u: &[f32], v: &[f32]) -> f32 {
    let length = u.len();
    let mut dp = SimdType::splat(0.0);
    for i in (0..length).step_by(SIMD_LANES) {
        let u_chunk = extract_simd_type_from_slice(u, i, length);
        let v_chunk = extract_simd_type_from_slice(v, i, length);
        dp += u_chunk * v_chunk;
    }
    dp.reduce_sum()
}

pub fn cosine_distance(u: &[f32], v: &[f32]) -> f32 {
    cfg_if! {
        if #[cfg(nightly)] {
            cosine_distance_simd(u, v)
        } else {
            cosine_distance_no_simd(u, v)
        }
    }
}

#[cfg(any(test, not(nightly)))]
pub fn cosine_distance_no_simd(u: &[f32], v: &[f32]) -> f32 {
    // want to calculate (a/|a| - b/|b|)^2
    // = a^2 / a^2 + b^2 / b^2 - 2ab/|a||b|
    // = 2 - 2cos
    let mut pp: f32 = 0.0;
    let mut qq: f32 = 0.0;
    let mut pq: f32 = 0.0;
    for (_u, _v) in u.iter().zip(v.iter()) {
        pp += _u * _u;
        qq += _v * _v;
        pq += _u * _v;
    }
    let ppqq = pp * qq;
    if ppqq.is_sign_positive() {
        2.0 - 2.0 * pq / ppqq.sqrt()
    } else {
        2.0
    }
}

#[cfg(nightly)]
pub fn cosine_distance_simd(u: &[f32], v: &[f32]) -> f32 {
    let length = u.len();
    let mut ppm = SimdType::default();
    let mut qqm = SimdType::default();
    let mut pqm = SimdType::default();
    for i in (0..length).step_by(SIMD_LANES) {
        let u_chunk = extract_simd_type_from_slice(u, i, length);
        let v_chunk = extract_simd_type_from_slice(v, i, length);
        ppm += u_chunk * u_chunk;
        qqm += v_chunk * v_chunk;
        pqm += u_chunk * v_chunk;
    }
    let pp = ppm.reduce_sum();
    let qq = qqm.reduce_sum();
    let pq = pqm.reduce_sum();
    let ppqq = pp * qq;
    if ppqq.is_sign_positive() {
        2.0 - 2.0 * pq / ppqq.sqrt()
    } else {
        2.0
    }
    // let ppqq = unsafe { fmul_fast(pp, qq) };
    // if ppqq.is_sign_positive() {
    //     unsafe { fsub_fast(2.0, fmul_fast(2.0, fdiv_fast(pq, sqrtf32(ppqq)))) }
    // } else {
    //     2.0
    // }
}

pub fn euclidean_distance(u: &[f32], v: &[f32]) -> f32 {
    cfg_if! {
        if #[cfg(nightly)] {
            euclidean_distance_simd(u, v)
        } else {
            euclidean_distance_no_simd(u, v)
        }
    }
}

#[cfg(any(test, not(nightly)))]
pub fn euclidean_distance_no_simd(u: &[f32], v: &[f32]) -> f32 {
    u.iter().zip(v.iter()).map(|(x, y)| (x - y).powi(2)).sum()
}

#[cfg(nightly)]
pub fn euclidean_distance_simd(u: &[f32], v: &[f32]) -> f32 {
    let length = u.len();
    let mut sum = SimdType::default();
    for i in (0..length).step_by(SIMD_LANES) {
        let u_chunk = extract_simd_type_from_slice(u, i, length);
        let v_chunk = extract_simd_type_from_slice(v, i, length);
        sum += power_simd_type(u_chunk - v_chunk);
    }
    sum.reduce_sum()
}

pub fn manhattan_distance(u: &[f32], v: &[f32]) -> f32 {
    cfg_if! {
        if #[cfg(nightly)] {
            manhattan_distance_simd(u, v)
        } else {
            manhattan_distance_no_simd(u, v)