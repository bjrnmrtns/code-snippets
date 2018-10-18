cmake_minimum_required(VERSION 3.12)

project(c++-code-structure)

set(CMAKE_CXX_STANDARD 17)

add_executable(${PROJECT_NAME}
    src/main.cpp
)

target_link_libraries(${PROJECT_NAME}
    PRIVATE
        componenta
)

add_subdirectory(src/componenta)
