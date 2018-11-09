cmake_minimum_required(VERSION 3.12)

project(concurrency)

set(CMAKE_CXX_STANDARD 17)

add_subdirectory(thirdparty)

add_executable(${PROJECT_NAME} src/concurrency.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE stlab)
