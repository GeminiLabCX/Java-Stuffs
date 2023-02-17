#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use annoy_rs::{
        wasm_exports::{SearchResultJs, *},
        IndexType,
    };
    use js_sys::{Array, Uint8Array};
    use wasm_bindgen::prelude::*;
    