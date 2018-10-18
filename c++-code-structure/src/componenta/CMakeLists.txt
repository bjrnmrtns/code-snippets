project(componenta)

add_library(${PROJECT_NAME}
    include/componenta/algorithm.hpp
    src/algorithm.cpp
)

target_include_directories(${PROJECT_NAME} PUBLIC include)
