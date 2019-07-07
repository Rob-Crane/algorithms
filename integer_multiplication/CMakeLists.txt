project(Integer_Multiplication)

cmake_minimum_required (VERSION 3.5)

add_definitions(-std=c++11)

set(CXX_FLAGS "-Wall")
set(CMAKE_CXX_FLAGS, "${CXX_FLAGS}")

set(sources src/mult_gradeschool.cpp src/test_mult_gradeschool.cpp)


#include_directories(/usr/local/include)

add_executable(test_gradeschool ${sources})
