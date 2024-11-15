//! See [llama-cpp-2](https://crates.io/crates/llama-cpp-2) for a documented and safe API.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(direct_link_llama))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(direct_link_llama)]
include!("../bindings.rs");
