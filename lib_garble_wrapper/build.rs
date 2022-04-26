// api_garble
// Copyright (C) 2O22  Nathan Prat

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use cmake::Config;
use rust_cxx_cmake_bridge::read_cmake_generated;

// NOTE: check git history for a "working" version using shared libs
// It worked locally but was a pain to deploy/package cf "DBUILD_SHARED_LIBS" below

fn main() {
    // BEFORE CMake: that will (among other things) generate rust/cxx.h that
    // is needed to compile src/rust_wrapper.cpp
    // ALTERNATIVELY we could add a git submodule for https://github.com/dtolnay/cxx/tree/master/include
    cxx_build::bridge("src/lib.rs").compile("lib-garble-wrapper");

    let mut cmake_config = Config::new(".");
    cmake_config.generator("Ninja");
    // NOTE: SHOULD NOT use shared libs
    // b/c it makes really messy to package/deploy/dockerize
    // Also makes it hard to debug and run tests from just this package while in parent crate.
    // (ie Undefined Reference)
    cmake_config.configure_arg("-DBUILD_SHARED_LIBS=OFF");
    // TODO use IPO/LTO, at least in Release
    cmake_config.build_target("rust_wrapper");
    // without this(default to true) cmake is run every time, and for some reason this thrashes the build dir
    // which causes to recompile from scratch every time(for eg a simple comment added in lib.rs)
    cmake_config.always_configure(false); // TODO always_configure

    // Use ccache/sccache based on the value of RUSTC_WRAPPER
    // NOTE: the logic is really basic but it works for our purposes(ie our CI and local dev)
    // TODO move this into custom crate(in same repo than rust_cxx_cmake_bridge?)
    // TODO do we reuse RUSTC_WRAPPER, or do we do like in CI and use custom ENV_CMAKE_CXX_COMPILER_LAUNCHER ?
    // ie do we couple cpp and rust or not?
    //
    // let rustc_wrapper = std::env::var("RUSTC_WRAPPER").unwrap_or("".to_string());
    // println!("cargo:debug=rustc_wrapper={:?}", rustc_wrapper);
    // // make sure it works both for ccache and sccache
    // // NOTE: if not ccache/sccache we ignore it and do nothing; eg it could by a custom rustc_wrapper.sh
    // // TODO? handle distcc the same way?
    // if rustc_wrapper.contains("ccache") {
    //     println!("cargo:info=rustc_wrapper is ccache/sccache");
    //     cmake_config.configure_arg(format!("-DCMAKE_CXX_COMPILER_LAUNCHER={}", rustc_wrapper));
    //     cmake_config.configure_arg(format!("-DCMAKE_C_COMPILER_LAUNCHER={}", rustc_wrapper));
    // }
    //
    // cf https://github.com/Interstellar-Network/gh-actions/blob/main/prepare/action.yml
    let env_cxx_compiler_launcher =
        std::env::var("ENV_CMAKE_CXX_COMPILER_LAUNCHER").unwrap_or("".to_string());
    // TODO remove println!
    println!(
        "cargo:warning=env_cxx_compiler_launcher: {}",
        env_cxx_compiler_launcher
    );
    if env_cxx_compiler_launcher.contains("ccache") {
        // TODO remove println!
        println!("cargo:warning=env_c_compiler_launcher is ccache/sccache");
        cmake_config.configure_arg(format!(
            "-DCMAKE_CXX_COMPILER_LAUNCHER={}",
            env_cxx_compiler_launcher
        ));
    }
    let env_c_compiler_launcher =
        std::env::var("ENV_CMAKE_C_COMPILER_LAUNCHER").unwrap_or("".to_string());
    // TODO remove println!
    println!(
        "cargo:warning=env_c_compiler_launcher: {}",
        env_c_compiler_launcher
    );
    if env_c_compiler_launcher.contains("ccache") {
        // TODO remove println!
        println!("cargo:warning=env_c_compiler_launcher is ccache/sccache");
        cmake_config.configure_arg(format!(
            "-DCMAKE_C_COMPILER_LAUNCHER={}",
            env_c_compiler_launcher
        ));
    }

    let rust_wrapper = cmake_config.build();

    // rust_wrapper.display() = /home/.../api_garble/target/debug/build/lib-garble-wrapper-XXX/out
    // but the final lib we want(eg librust_wrapper.a) is below in build/src/
    // TODO remove? this is done as part of the loop below
    println!(
        "cargo:rustc-link-search=native={}/build/src/",
        rust_wrapper.display()
    );
    println!("cargo:rustc-link-lib=static=rust_wrapper");

    // target/debug/build/lib-garble-wrapper-XXX/out/build/src/cmake_generated_libs
    let cmake_generated_libs_str = std::fs::read_to_string(
        &format!("/{}/build/src/cmake_generated_libs", rust_wrapper.display()).to_string(),
    )
    .unwrap();

    read_cmake_generated(&cmake_generated_libs_str);

    // TODO get the system libs using ldd?
    // println!("cargo:rustc-link-lib=readline");

    // But careful, we MUST recompile if the .cpp, the .h or any included .h is modified
    // and using rerun-if-changed=src/lib.rs make it NOT do that
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=deps/lib_garble/src/");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
}
