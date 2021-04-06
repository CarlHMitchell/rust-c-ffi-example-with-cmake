cmake_minimum_required(VERSION 3.17)
project(corrosion_experiments C)

set(CMAKE_C_STANDARD 11)

# Set an env var for finding the project root.
set(FFI_PROJECT_ROOT ${CMAKE_SOURCE_DIR})

add_subdirectory(rust_lib)

add_subdirectory(c_app)
add_subdirectory(c_lib)
