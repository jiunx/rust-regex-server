#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub use crate::bindings::*;

mod bindings;

pub const PCRE2_UNSET: usize = ::std::usize::MAX;
