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
            &[0.0, 0.416_088_22, 0.551_752_3, 0.734_209_54, 0.759_296_1],
        );
    }

    #[test]
    fn sanity_tests_euclidean() {
        sanity_tests_inner(
            IndexType::Euclidean,
            &[
                1.522_306_6,
                -1.520_689_5,
                0.226_999_3,
                0.408_149_27,
                0.640_252_8,
            ],
            &[0, 84, 20, 49, 94],
            &[0.0, 0.934_874_3, 1.105_167_6, 1.105_779_3, 1.129_980_7],
        );
    }

    #[test]
    fn sanity_tests_manhattan() {
        sanity_tests_inner(
            IndexType::Manhattan,
            &[
                -0.794_453_5,
                0.907_682_3,
                1.816_441_7,
                -0.783_995_8,
                -0.655_002_24,
            ],
            &[0, 34, 89, 83, 41],
            &[0.0, 1.683_599_5, 1.797_636, 2.139_925, 2.144_656_2],
        );
    }

    #[test]
    fn sanity_tests_dot() {
        sanity_tests_inner(
            IndexType::Dot,
            &[
                -1.295_846_3,
                0.268_831_16,
                0.424_712_87,
                0.479_184_27,
                0.562_680_07,
            ],
            &[42, 89, 0, 40, 61],
            &[3.553_952_7, 3.538_242_3, 3.151_576, 3.045_288_3, 2.615_417],
        );
    }

    fn sanity_tests_inner(
        index_type: IndexType,
        expected_item3_vec: &[f32],
        expected_id_list: &[u64],
        expected_distance_list: &[f32],
    ) {
        let filepath = format!("../tests/index.{index_type}.{TEST_INDEX_DIM}d.ann");
        let filepath_cstring = CString::new(filepath).unwrap();
        unsafe {
            let index = load_annoy_index(
                filepath_cstring.into_raw() as *const c_char,
                TEST_INDEX_DIM as i32,
                index_type as u8,
            );
            let dim = get_dimension(index);
            assert_eq!(dim, TEST_INDEX_DIM as i32);
            let v3_raw = alloc(Layout::array::<f32>(dim as usize).unwr