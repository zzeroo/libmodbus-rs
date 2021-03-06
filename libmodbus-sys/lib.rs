#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Fixes 'note: 128-bit integers don't currently have a known stable ABI' https://github.com/rust-lang/rust-bindgen/issues/1549
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
