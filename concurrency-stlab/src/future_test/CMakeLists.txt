project(future_test)
add_executable(${PROJECT_NAME} src/future_test.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE stlab)
