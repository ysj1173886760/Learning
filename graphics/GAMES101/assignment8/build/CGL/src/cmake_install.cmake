# Install script for directory: /home/sheep/Learning/graphics/assignment8/CGL/src

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/usr/local")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/sheep/Learning/graphics/assignment8/build/CGL/src/libCGL.a")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/CGL" TYPE FILE FILES
    "/home/sheep/Learning/graphics/assignment8/CGL/src/CGL.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/vector2D.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/vector3D.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/vector4D.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/matrix3x3.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/matrix4x4.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/quaternion.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/complex.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/color.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/osdtext.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/viewer.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/base64.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/tinyxml2.h"
    "/home/sheep/Learning/graphics/assignment8/CGL/src/renderer.h"
    )
endif()

