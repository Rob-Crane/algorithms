project(Integer_Multiplication)

cmake_minimum_required (VERSION 3.5)

add_definitions(-std=c++11)

set(CXX_FLAGS "-Wall")
set(CMAKE_CXX_FLAGS, "${CXX_FLAGS}")

set(gradeschool_src src/mult_common.cpp src/mult_gradeschool.cpp src/test_mult_gradeschool.cpp)

add_executable(test_gradeschool ${gradeschool_src})
