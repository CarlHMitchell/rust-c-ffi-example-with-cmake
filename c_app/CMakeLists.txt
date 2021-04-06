cmake_minimum_required(VERSION 3.17)

project(c_app C)

add_executable(c_app
    src/main.c
    )

target_link_libraries(c_app
    PUBLIC
    rust_lib
    )
