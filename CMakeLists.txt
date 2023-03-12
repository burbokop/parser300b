cmake_minimum_required(VERSION 3.18.4)
project(parser300b CXX)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_LIST_DIR}/Cargo.toml)

add_executable(tests
    ${CMAKE_CURRENT_LIST_DIR}/src/test.cpp
)

target_sources(parser300b
PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/src/lib.h
)

target_link_libraries(tests
    parser300b
)
