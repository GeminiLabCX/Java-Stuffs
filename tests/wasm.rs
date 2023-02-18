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
  