#![deny(warnings)]
#![cfg_attr(nightly, feature(portable_simd))]
#![cfg_attr(nightly, feature(core_intrinsics))]
#![cfg_attr(nightly, feature(test))]

#[macro_use]
extern crate cfg_if;

pub(crate) mod internals