cmake_minimum_required(VERSION 3.19)
cmake_policy(SET CMP0048 NEW)
project(longbridge)

include(FetchContent)
fetchcontent_declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.2.1
)
fetchcontent_makeavailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH Cargo.toml CRATES longbridge-c)

include_directories(csrc/)

add_executable(test test/main.c)
target_link_libraries(test longbridge-c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(test ncurses)
endif()

add_executable(test_cpp test/main.cpp)
target_link_libraries(test_cpp longbridge-c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(test_cpp ncurses)
endif()
