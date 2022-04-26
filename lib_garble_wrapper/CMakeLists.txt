cmake_minimum_required(VERSION 3.22)

project(rust_wrapper)

################################################################################
# DO NOT use "add_subdirectory(deps/lib_garble/)"
# For some reasons, when using "add_subdirectory", transitive deps which use Conan
# are NOT listed by our CMake function "export_all_target_libs" and we get
# "undefined symbols" for eg libprotobuf etc
option(interstellar_lib_garble_BUILD_TESTS "Build tests" OFF)  # default ON
# TOREMOVE? FAIL: "is already used to build a source directory"
include(deps/lib_garble/CMakeLists.txt)

################################################################################

include(deps/lib_garble/deps/cmake/coverage.cmake)
include(deps/lib_garble/deps/cmake/options.cmake)
include(deps/lib_garble/deps/cmake/sanitizers.cmake)
include(deps/lib_garble/deps/cmake/compile_flags.cmake)
include(deps/lib_garble/deps/cmake/export_libs.cmake)

################################################################################

add_subdirectory(src)

################################################################################
