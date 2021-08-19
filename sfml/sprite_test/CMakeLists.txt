cmake_minimum_required(VERSION 3.10)

set(PROJECT_NAME sprite_test)
project(${PROJECT_NAME})

find_package(OpenGL)

include_directories("${CMAKE_SOURCE_DIR}/include")

file(GLOB source_files "${CMAKE_SOURCE_DIR}/src/*.cpp")
set(SOURCES ${source_files})
set(CMAKE_CXX_STANDARD 11)

# Detect and add SFML
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/../cmake_modules" ${CMAKE_MODULE_PATH})
find_package(SFML 2 REQUIRED COMPONENTS audio graphics window system)

add_executable(${PROJECT_NAME} ${SOURCES})
target_link_libraries(${PROJECT_NAME} sfml-system sfml-window sfml-graphics sfml-audio)

if (UNIX)
   target_link_libraries(${PROJECT_NAME} pthread)
endif (UNIX)