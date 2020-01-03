cmake_minimum_required(VERSION 3.12)
project(bor)

add_library(bor STATIC
    src/bor.c
)

target_include_directories(bor PUBLIC include)
install(TARGETS bor DESTINATION .)
