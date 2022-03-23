cmake_minimum_required(VERSION 3.22)

project(rust_wrapper)

################################################################################

include(deps/lib_garble/deps/cmake/coverage.cmake)
include(deps/lib_garble/deps/cmake/options.cmake)
include(deps/lib_garble/deps/cmake/sanitizers.cmake)
include(deps/lib_garble/deps/cmake/compile_flags.cmake)
include(deps/lib_garble/deps/cmake/export_libs.cmake)

################################################################################

option(interstellar_garble_BUILD_TESTS "Build tests" OFF)  # default ON
add_subdirectory(deps/lib_garble/)

################################################################################

add_subdirectory(src)

################################################################################
