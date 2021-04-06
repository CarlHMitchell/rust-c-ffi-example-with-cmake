cmake_minimum_required(VERSION 3.17)

project(c_lib C)

add_library(c_lib STATIC
    src/c_lib.c
    )

target_include_directories(c_lib
    PUBLIC
    inc/
    )

target_link_libraries(c_lib
    PRIVATE
    )

