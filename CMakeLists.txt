cmake_minimum_required(VERSION 3.20)

project(foo)

add_library(foo STATIC src/foo.c)
target_include_directories(foo PUBLIC include)
