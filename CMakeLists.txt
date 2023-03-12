cmake_minimum_required(VERSION 3.18.4)
project(parser300b CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED On)

message("CMAKE_CXX_COMPILER: ${CMAKE_CXX_COMPILER}")

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_LIST_DIR}/Cargo.toml)

target_sources(parser300b
PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/src/lib.h
)

include(${CMAKE_CURRENT_LIST_DIR}/tests.cmake)
