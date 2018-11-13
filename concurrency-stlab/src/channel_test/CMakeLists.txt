project(channel_test)
add_executable(${PROJECT_NAME} src/channel_test.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE stlab)
