#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use annoy_rs::{
        wasm_exports::{SearchResultJs, *},
        IndexType,
    };
    use js_sys::{Array, Uint8Array};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    static HOLE_INDEX_BYTES: &[u8] = include_bytes!("hole.10d.ann");

    #[wasm_bindgen_test]
    fn hole_tests() {
        let u8a = Uint8Array::new_with_length(HOLE_INDEX_BYTES.len() as u32);
        u8a.copy_from(HOLE_INDEX_BYTES);
        let index = load_index(&u8a, 10, IndexType::Angular).unwrap();
        assert_eq!(index.dimension, 10, "Wrong dimension {}", index.dimension);
        assert_eq!(index.size, 1001, "Wrong size {}", index.size);
        let v1 = vec![
            0.10471842,
            0.55223828,
            0.44094049,
            0.98384884,
            0.22485616,
            -0.79840456,
            -1.78999692,
            -1.11747558,
            0.05733591,
            1.35356555,
        ];
        let v1_