#![allow(non_camel_case_types)]

use std::ptr;
use std::slice;
use std::str;

use libc::{c_int, c_void, size_t};

pub fn captures<'a, 'b>(pattern: &'a str, text: &'b str) -> Vec<Vec<&'b str>> {
    let mut captures = Vec::new();
    let code = unsafe {
        pcre2_compile_8(
            pattern.as_ptr(),
            pattern.len(),
            0x00020000 | 0x00080000,
            &mut 0,
            &mut 0,
            ptr::null_mut(),
        )
    };
    let err = unsafe { pcre2_jit_compile_8(code, 0x00000001) };
    if err < 0 {
        panic!("pcre2_jit_compile_8 failed with error: {:?}", err);
    }
    let match_data = unsafe { pcre2_match_data_create_from_pattern_8(code, ptr::null_mut()) };
    if match_data.is_null() {
        panic!("could not allocate match_data");
    }
    let ovector = unsafe { pcre2_get_ovector_pointer_8(match_data) };
    if ovector.is_null() {
        panic!("could not get ovector");
    }
    let ovector_count = unsafe { pcre2_get_ovector_count_8(match_data) };
    if ovector_count == 0 {
        panic!("ovector_count is 0");
    }

    let mut err = 0;
    let mut start = 0;
    let ovec = unsafe { slice::from_raw_parts(ovector, ovector_count as usize * 2) };

    while err != -1 {
        let mut matched = Vec::new();
        err = unsafe {
            pcre2_jit_match_8(
                code,
                text.as_ptr(),
                text.len(),
                start,
                0x40000000,
                match_data,
                ptr::null_mut(),
            )
        };

        if err == -1 {
            break;
        } else if err < 0 {
            panic!("unknown error code: {:?}", err)
        } else {
            for i in 0..ovector_count {
                let s = match ovec.get((i * 2) as usize) {
                    None => continue,
                    Some(&s) if s == ::std::usize::MAX => continue,
                    Some(&s) => s,
                };
                let e = match ovec.get((i * 2 + 1) as usize) {
                    None => continue,
                    Some(&e) if e == ::std::usize::MAX => continue,
                    Some(&e) => e,
                };
                if i == 1 {
                    start = e;
                }
                matched.push(&text[s..e]);
            }
        }
        captures.push(matched);
    }
    captures
}

// ffi wrapper pcre2

type code = c_void;

type match_data = c_void;

type compile_context = c_void;

type general_context = c_void;

type match_context = c_void;

extern "C" {
    fn pcre2_compile_8(
        pattern: *const u8,
        len: size_t,
        options: u32,
        error_code: *mut c_int,
        error_offset: *mut size_t,
        context: *mut compile_context,
    ) -> *mut code;

    fn pcre2_match_data_create_from_pattern_8(
        code: *const code,
        context: *mut general_context,
    ) -> *mut match_data;

    fn pcre2_get_ovector_pointer_8(match_data: *mut match_data) -> *mut size_t;

    fn pcre2_jit_compile_8(code: *const c_void, options: u32) -> c_int;

    fn pcre2_jit_match_8(
        code: *const code,
        subject: *const u8,
        length: size_t,
        startoffset: size_t,
        options: u32,
        match_data: *mut match_data,
        match_context: *mut match_context,
    ) -> c_int;

    pub fn pcre2_get_ovector_count_8(arg1: *mut match_data) -> u32;

}
