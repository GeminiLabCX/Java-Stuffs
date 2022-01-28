#[cfg(test)]
mod tests {
    use annoy_rs_ffi::{annoy_rs::*, *};
    use libc::c_char;
    use std::alloc::{alloc, Layout};
    use std::ffi::CString;
    use std::ptr;
    use std::slice;

    const F32_PRECISION: usize = 2;
    const TEST_INDEX_DIM: usize = 5;
    const TEST_NODE_COUNT: usize = 100;

    #[test]
    fn sanity_tests_angular() {
        sanity_tests_inner(
            IndexType::Angular,
            &[
                -0.388_461_32,
                0.879_120_65,
                0.058_009_166,
                0.866_426_65,
                0.402_518_24,
            ],
            &[0, 4, 37, 61, 29],
            &[0.0, 0.416_