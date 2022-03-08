// #![no_std]
// https://github.com/substrate-developer-hub/substrate-module-template/blob/master/HOWTO.md#forgetting-cfg_attr-for-no_std
#![cfg_attr(not(feature = "std"), no_std)]

pub use cxx;

#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("lib-garble-wrapper/src/rust_wrapper.h");

        type GarbleWrapper;

        fn new_garble_wrapper() -> UniquePtr<GarbleWrapper>;
        fn GarbleSkcdToBuffer(&self, skcd_input_path: &str) -> Vec<u8>;
    }
}

#[cfg(test)]
mod tests {
    use crate::ffi;
    use std::fs::File;
    use tempfile::Builder;

    // TODO fix undefined reference to `GenerateSegmentedDigitCache()' aaa
    // WARNING linker errors when trying to test in repo api_circuits
    // #[test]
    // fn generate_display_skcd_basic() {
    //     let circuit_gen_wrapper = ffi::new_circuit_gen_wrapper();

    //     let width = 224;
    //     let height = 96;

    //     let tmp_dir = Builder::new()
    //         .prefix("interstellar-circuit_routes")
    //         .tempdir()
    //         .unwrap();

    //     let file_path = tmp_dir.path().join("output.skcd.pb.bin");

    //     // TODO make the C++ API return a buffer?
    //     circuit_gen_wrapper.GenerateDisplaySkcd(
    //         file_path.as_os_str().to_str().unwrap(),
    //         width,
    //         height,
    //     );

    //     // TODO test file_path size? just exists?
    //     assert!(file_path.exists());
    //     assert_eq!(file_path.metadata().unwrap().len(), 4242);
    // }
}
