cmake_minimum_required(VERSION 3.17)


message("FFI_PROJECT_ROOT = ${FFI_PROJECT_ROOT}")
set(ENV{FFI_PROJECT_ROOT} ${FFI_PROJECT_ROOT})

# It's recommended to build and install Corrosion locally.
# EG, for me (carl):
#set(Corrosion_DIR "/home/carl/bin/corrosion/lib/cmake/Corrosion/")
#find_package(Corrosion REQUIRED)

configure_file (
    "${FFI_PROJECT_ROOT}/rust_lib/build.rs.in"
    "${FFI_PROJECT_ROOT}/rust_lib/build.rs"
)

# But it's also possible (and easy) to use the FetchContent module to download
#   and build it automatically. This makes building slower if the cmake build
#   directory gets deleted.
include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/AndrewGaspar/corrosion.git
    GIT_TAG origin/master # Optionally specify a version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# This could point to a subdirectory.
corrosion_import_crate(MANIFEST_PATH Cargo.toml)

target_include_directories(rust_lib
    INTERFACE
    inc
    )

# Wrapper for bindgen
target_sources(rust_lib
    INTERFACE
    wrapper.h
    )

target_link_libraries(rust_lib
    INTERFACE
    c_lib
    )
