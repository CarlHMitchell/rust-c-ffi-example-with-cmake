/*
 * Bindings for C functions.
 */

use libc::*;

extern "C" {
    pub fn c_double_input(input: i32) -> i32;
}

extern "C" {
    pub fn c_increment_int_array(length: size_t, array: *mut i32);
}
