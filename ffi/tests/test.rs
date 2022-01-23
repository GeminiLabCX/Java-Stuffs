#[cfg(test)]
mod tests {
    use annoy_rs_ffi::{annoy_rs::*, *};
    use libc::c_char;
    use std::alloc::{alloc, Layout};
    use std::ffi::CString;
    use std::ptr;
    use std::slice;

    const F32