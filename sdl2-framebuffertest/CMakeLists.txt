cmake_minimum_required(VERSION 3.12)
project(sdl2-framebuffertest)

find_package(SDL2 REQUIRED)

add_executable(sdl2-framebuffertest
    src/sdl2-framebuffertest.cpp
)

target_link_libraries(sdl2-framebuffertest ${SDL2_LIBRARIES})
