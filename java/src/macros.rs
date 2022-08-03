macro_rules! ffi_fn {
    (fn $name:ident($($arg:ident: $arg_ty:ty),*,) -> $ret:ty $body:block) => {
        ffi_fn!(fn $name($($arg: $arg_ty),*) -> $ret $body);
    };
    (fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty $body:block) => {
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[no_mangle]
        pub extern "system" fn $name($($arg: $arg_ty),*) -> $ret {
            use ::std::io::{self, Write};
            use ::std::panic::{self, AssertUnwindSafe};
            use ::libc::abort;
            match panic::catch_unwind(AssertUnwindSafe(move || $body)) {
                Ok(v) => v,
    