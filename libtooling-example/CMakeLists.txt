cmake_minimum_required(VERSION 3.12)

find_package(Clang REQUIRED)

add_executable(clang-bor src/clang-bor.cpp)
target_include_directories(clang-bor PUBLIC ${CLANG_INCLUDE_DIRS})
target_link_libraries(clang-bor PRIVATE clangTooling)
