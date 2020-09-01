project(write-png)
find_package(PNG)

add_executable(write-png
    src/main.cpp)

target_link_libraries(write-png PRIVATE PNG::PNG)


