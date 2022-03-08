use cmake::Config;
use glob::glob;
use std::path::Path;

fn main() {
    // TODO can we make it work with SHARED libs?
    let garble_lib = Config::new("../deps/lib_garble/")
        // TODO use IPO/LTO, at least in Release
        .configure_arg("-GNinja")
        // NOTE: can NOT really link using static libs
        // b/c the target circuit_lib by itself does nothing, so pretty much everything end up optimized out by the linker
        // and we end up with A LOT of "undefined reference"
        .configure_arg("-DBUILD_SHARED_LIBS=ON")
        // TODO CMAKE_EXPORT_COMPILE_COMMANDS? instead of glob?
        // TODO target "all"?
        .build_target("garble_helper")
        // without this(default to true) cmake is run every time, and for some reason this thrashes the build dir
        // which causes to recompile from scratch every time(for eg a simple comment added in lib.rs)
        .always_configure(false)
        .build();
    // CHECK
    println!("### garble_lib.display: {}", garble_lib.display());
    // println!("cargo:rustc-link-search=native={}", garble_lib.display()); // USELESS
    // rust_deps.display() = .../target/debug/build/lib-server-2cb77547fe2ce2c4/out
    // but .a static libs are below in out/build
    // TODO can we instead pass eg CMAKE_BINARY_DIR and have all .a in the same dir(no subdir)?
    println!(
        "cargo:rustc-link-search=native={}/build/src/",
        garble_lib.display()
    );

    // will link to ALL the libs built by CMake...
    // This is not ideal but at least:
    // - it works
    // - it avoids hardcoding a whole list of dependencies of "garble_lib"
    for entry in glob(&format!("/{}/**/*.a", garble_lib.display()).to_string())
        .unwrap()
        .chain(glob(&format!("/{}/**/*.so", garble_lib.display()).to_string()).unwrap())
    {
        match entry {
            Ok(static_lib_path) => {
                println!("{:?}", static_lib_path.display());

                // /home/.../api_circuits/target/debug/build/lib-garbke-wrapper-49025516ce40925e/out/build/_deps/abseil-build/absl/types/libabsl_bad_optional_access.a
                // we want: absl_bad_optional_access
                let liblib_name = static_lib_path.file_stem().unwrap();
                let liblib_name_str: String = liblib_name.to_str().unwrap().into();
                let lib_name_str = liblib_name_str.strip_prefix("lib").unwrap();
                println!("## lib_name: {:?}", lib_name_str);
                let dir = static_lib_path.parent().unwrap();
                println!("## dir: {:?}", dir);

                println!("cargo:rustc-link-search=native={}", dir.display());
                println!("cargo:rustc-link-lib={}", lib_name_str);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    cxx_build::bridge("src/lib.rs")
        .file("src/rust_wrapper.cpp")
        // C++20 else:
        // error: no type named 'string_view' in namespace 'std'
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-std=c++2a")
        // TODO use a dynamic path from CMake above
        .include("src")
        .include("../deps/lib_garble/src/")
        .compile("lib-garble-wrapper");

    // TODO? but careful, we MUST recompile if the .cpp, the .h or any included .h is modified
    // and using rerun-if-changed=src/lib.rs make it NOT do that
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/rust_wrapper.h");
    println!("cargo:rerun-if-changed=src/rust_wrapper.cpp");
    println!("cargo:rerun-if-changed=../deps/lib_garble/src/");
    println!("cargo:rerun-if-changed=build.rs");
}
