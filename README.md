# Intro
This project is a set of examples of FFI between Rust and C.  
It uses CMake to build the C, and CMake to call Cargo to build the Rust.  
This is not intended to be a Rust, C, or CMake tutorial, merely an example of
using them together. That said, comments should explain to C programmers what
the Rust code is doing.

Cargo, rustc, CMake, and a C compiler must be installed.  
To install Rust (Cargo and rustc), follow the instructions at https://www.rust-lang.org/tools/install  
Get CMake from https://cmake.org/download/ or your distribution's package manager (3.17+ needed)
GCC (A C compiler) can also been used, and was used to test this project. It can be downloaded from https://gcc.gnu.org/  
LLVM & Clang (a C compiler) can be downloaded from https://releases.llvm.org/download.html  


To cross-compile, both a C and Rust cross-compiler are needed. For Rust,
`rustup target add <triple>` where <triple> is a target from the list at
https://doc.rust-lang.org/nightly/rustc/platform-support.html. For C, that's
probably a cross-compiler and toolchain from your distro's package manager.
CMake will need to define an appropriate `Rust_CARGO_TARGET` value, eg
`-DRust_CARGO_TARGET=armv7-unknown-linux-gnueabihf` in addition to any C/C++
target flags needed. Adding cross-compiler support is not a goal of this
project, and would require some extra scripts to call `cmake` with appropriate
options.

NOTE: This has so far only been tested on Debian Bullseye. It should work for
any Linux system, not sure about Mac or Windows.


# Useful Links
### Rust Documentation
[The Rust Book](https://doc.rust-lang.org/stable/book/)  
[The Rust Reference](https://doc.rust-lang.org/reference/introduction.html)  
[Reference chapter on build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)  
[Rustdoc book](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)  

### FFI tutorials and examples
https://jakegoulding.com/rust-ffi-omnibus/  
https://github.com/alexcrichton/rust-ffi-examples  
https://doc.rust-lang.org/nomicon/ffi.html  

### Binding generators
https://github.com/eqrion/cbindgen  
https://github.com/eqrion/cbindgen/blob/master/docs.md  
https://rust-lang.github.io/rust-bindgen/introduction.html  

### CMake helper for Cargo (Rust's build system)
https://github.com/AndrewGaspar/corrosion

### Rust's Library repository
https://crates.io/

### CMake documentation
https://cmake.org/cmake/help/v3.17/index.html

### Cross compiling Rust
https://github.com/japaric/rust-cross
https://rust-lang.github.io/rustup/cross-compilation.html

### Helpful links for learning Rust
[Rustlings](https://github.com/rust-lang/rustlings) provides small exercises, and is a good intro.

The [Rust Book](https://doc.rust-lang.org/book/index.html) linked above as well, is comprehensive if theory-focused.

[Rust-by-example](https://doc.rust-lang.org/rust-by-example/index.html) is exactly what the title says.

[Learning Rust with Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
is all about linked lists. Linked lists aren't something you usually need to
implement yourself, but they're the hardest of the data structures taught to
undergraduates to implement safely.

[Dancing Links in Rust](https://ferrous-systems.com/blog/dlx-in-rust/) provides
an example of one of the rare algorithms that really needs a linked list. And
two-dimensional linked list, at that!

https://ferrous-systems.com/blog/omg-wtf-rs-resources-to-help-you-get-started-with-rust/

[The Rustonomicon: The Dark Arts of Unsafe Rust](https://doc.rust-lang.org/stable/nomicon/)
is a good reference to the use of Unsafe in Rust. Since C is wildly unsafe it's
very common to need to make safe wrappers around unsafe C code.

[The Embedded Rust Book](https://doc.rust-lang.org/stable/embedded-book/) is a
guide to Rust on bare-metal (no OS, no standard library) development in Rust.

