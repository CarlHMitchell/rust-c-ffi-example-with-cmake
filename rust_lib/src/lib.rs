#![crate_type = "staticlib"]
#![warn(missing_docs)]
//! This crate is an example for interacting with C via Foreign Function Interface.
//! Use `cargo doc` from the `rust_lib` directory to generate HTML documentation.

/* Usually one would need to suppress some errors due to Rust's enforcing
 *  a different style than C. For this example, I've just kept to a Rust-like
 *  style.
 */
mod bindings;

// The libc crate provides some handy bindings for c standard library
extern crate libc;

// c characters
use libc::c_char;
// Convert c strings (char*) to Rust &str
use std::ffi::CStr;
// Convert c strings (char*) to Rust String
use std::ffi::CString;
// Binding from size_t to usize
use libc::size_t;
// Slice manipulation.
use std::slice;
// Used to do value conversions while consuming the input value. Easier error handling.
use std::convert::From;
// Use the bindings from c_lib generated by bindgen
use crate::bindings::{c_double_input, c_increment_int_array};
// One of the examples from the Rust FFI Omnibus uses a HashMap
use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn println_hello_world() {
    //! Print a line to stdout. Newline is included. print!() wouldn't add that.
    println!("Hello, World from Rust!");
}

#[no_mangle]
pub extern "C" fn contains_hotdog(s: *const c_char) -> bool {
    /*!
    This is a doc comment block that applies to the surrounding scope.
      That means the whole of contains_hotdog().

    Rustdoc generates documentation only for functions, structs, traits, structs,
      etc, but not to individual statements.

    This function is a silly reference to a TV show.
    It takes in a raw pointer to some C `char` or `char`s, and returns a `bool`.
    */
    //! This is a line doc comment that applies to the surrounding scope.
    assert!(!s.is_null());
    /*
    c_str gets its type inferred, but the following line could also be written
    `let c_str: &CStr = unsafe {`
    Converting a pointer into a CStr reference requires dereferencing the
        pointer, which is unsafe.
     */
    let c_str = unsafe {
        CStr::from_ptr(s)
    };
    let rust_str = c_str.to_str().unwrap();
    rust_str.contains("hotdog")
}

#[no_mangle]
/**
This is a doc comment block that applies to the following scope.
*/
pub extern "C" fn increment_array(length: size_t, array: *mut i32) {
    let numbers = unsafe {
        assert!(!array.is_null());
        slice::from_raw_parts_mut(array, length as usize)
    };

    for number in numbers {
        *number += 1;
    }
}

#[no_mangle]
/// This is a line doc comment that applies to the following line.
pub extern "C" fn increment_array_via_c(length: size_t, array: *mut i32) {
    // Calling C functions is always unsafe, since they can do anything.
    unsafe { c_increment_int_array(length, array) }
}

#[no_mangle]
/// Double the value of an `int32_t`.
pub extern "C" fn double_input(input: i32) -> i32 {
    // Calling C functions is always unsafe, since they can do anything.
    unsafe { c_double_input(input) }
}

#[no_mangle]
/// Count how many Unicode Extended Grapheme Clusters are in a string
pub extern "C" fn how_many_characters(s: *const c_char) -> u32 {
    // Check for NULL
    assert!(!s.is_null());
    let c_str = unsafe {
        /*
        Dereferencing raw pointers (* on a type) is unsafe.
        Even if not NULL, they could point to memory that has already been freed
          or was never properly allocated at all. Or to other invalid memory.
         */
        CStr::from_ptr(s)
    };

    // Convert the c_str to a rust &str (string slice), panicking on failure
    let rust_str = c_str.to_str().unwrap();
    // Count the Unicode extended grapheme clusters in the string
    rust_str.chars().count() as u32
}

#[no_mangle]
/// Count how many bytes are in a string.
pub extern "C" fn how_many_bytes(s: *const c_char) -> u32 {
    // Check for NULL
    assert!(!s.is_null());
    let c_str = unsafe {
        // Dereferencing raw pointers is unsafe
        CStr::from_ptr(s)
    };

    // Convert the c_str to a rust &str (string slice), panicking on failure
    let rust_str = c_str.to_str().unwrap();
    // Count the 8-bit bytes in the string. Unlike C, Rust uses 8-bit bytes.
    rust_str.len() as u32
}

#[no_mangle]
/// Return a string to C. The string is owned by Rust, and must be freed by Rust
pub extern "C" fn return_hello_world() -> *mut c_char {
    let greeting = String::from("Hello, World from a Rust string!");
    // The following will panic if the creation of a C string fails
    let c_str_greeting = CString::new(greeting).unwrap();
    // Converting an owned string into a raw pointer is *NOT* unsafe.
    c_str_greeting.into_raw()
}

#[no_mangle]
/// Free a string owned by Rust
pub extern "C" fn free_rust_allocated_string(s: *mut c_char) {
    // Dereferencing a raw pointer is unsafe
    unsafe {
        // Check for NULL, but don't abort then, just do nothing.
        if s.is_null() {
            return;
        }
        // s wasn't null, so convert it to an owned string, and let it go out
        //   of scope. That automatically calls free().
        CString::from_raw(s)
    };
}

// https://jakegoulding.com/rust-ffi-omnibus/slice_arguments/
#[no_mangle]
/// Add all the even numbers in an input array from C
pub extern "C" fn sum_of_even(n: *const u32, len: size_t) -> u32 {
    assert!(!n.is_null());
    let numbers = unsafe {
        slice::from_raw_parts(n, len as usize)
    };

    numbers.iter().filter(|&v| v % 2 == 0).sum()
}

// https://jakegoulding.com/rust-ffi-omnibus/tuples/
/// A Rust function that accepts a tuple
fn flip_things_around_rust(tup: (u32, u32)) -> (u32, u32) {
    let (a, b) = tup;
    (b + 1, a - 1)
}

// A struct that can be passed between C and Rust
#[repr(C)]
/// A Tuple of two u32s. To C, this is a `struct`.
pub struct Tuple {
    x: u32,
    y: u32,
}

// Conversion functions
impl From<(u32, u32)> for Tuple {
    fn from(tup: (u32, u32)) -> Tuple {
        Tuple { x: tup.0, y: tup.1 }
    }
}

impl From<Tuple> for (u32, u32) {
    fn from(tup: Tuple) -> (u32, u32) {
        (tup.x, tup.y)
    }
}

#[no_mangle]
/// The exported C method for flip_things_around_rust defined above.
pub extern "C" fn flip_things_around(tup: Tuple) -> Tuple {
    flip_things_around_rust(tup.into()).into()
}

/// https://jakegoulding.com/rust-ffi-omnibus/objects/ struct object
pub struct ZipCodeDatabase {
    population: HashMap<String, u32>,
}

impl ZipCodeDatabase {
    fn new() -> ZipCodeDatabase {
        ZipCodeDatabase {
            population: HashMap::new(),
        }
    }

    fn populate(&mut self) {
        for i in 0..100_000 {
            let zip = format!("{:05}", i);
            self.population.insert(zip, i);
        }
    }

    fn population_of(&self, zip: &str) -> u32 {
        self.population.get(zip).cloned().unwrap_or(0)
    }
}

#[no_mangle]
/// Create a new ZipCodeDatabase HashMap structure
pub extern "C" fn zip_code_database_new() -> *mut ZipCodeDatabase {
    Box::into_raw(Box::new(ZipCodeDatabase::new()))
}

#[no_mangle]
/// Free an existing ZipCodeDatabase structure
pub extern "C" fn zip_code_database_free(ptr: *mut ZipCodeDatabase) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
/// Populate a ZipCodeDatabase structure with some dummy data
pub extern "C" fn zip_code_database_populate(ptr: *mut ZipCodeDatabase) {
    let database = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    database.populate();
}

#[no_mangle]
/// Get the population of a Zip Code in a ZipCodeDatabase
pub extern "C" fn zip_code_database_population_of(
    ptr: *const ZipCodeDatabase,
    zip: *const c_char,
) -> u32 {
    let database = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let zip = unsafe {
        assert!(!zip.is_null());
        CStr::from_ptr(zip)
    };
    let zip_str = zip.to_str().unwrap();
    database.population_of(zip_str)
}
