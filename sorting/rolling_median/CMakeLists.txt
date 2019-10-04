project(Rolling_Median)

cmake_minimum_required (VERSION 3.5)

add_definitions(-std=c++11)

set(CXX_FLAGS "-Wall")
set(CMAKE_BUILD_TYPE Debug)
set(CMAKE_CXX_FLAGS, "${CXX_FLAGS}")

set(test_src src/rolling_median.cpp src/test_rolling.cpp)
set(src src/rolling_median.cpp src/run_rolling.cpp)

add_executable(test_rolling ${test_src})
add_executable(rolling_median ${src})
