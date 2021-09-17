cmake_minimum_required(VERSION 3.10)

set(PROJECT_NAME test)
project(${PROJECT_NAME})

include_directories("${CMAKE_SOURCE_DIR}/include")

find_package(OpenGL)

set(CMAKE_CXX_STANDARD 17)

message(STATUS "source file list: ${source_files}")

add_executable(${PROJECT_NAME} glad.c test.cpp)

target_link_libraries(${PROJECT_NAME} glfw Xrandr Xi pthread X11 ${CMAKE_DL_LIBS})