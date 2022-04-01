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

// #![no_std]
// https://github.com/substrate-developer-hub/substrate-module-template/blob/master/HOWTO.md#forgetting-cfg_attr-for-no_std
#![cfg_attr(not(feature = "std"), no_std)]

pub use cxx;

#[cxx::bridge]
pub mod ffi {

    struct StrippedCircuit {
        circuit_buffer: Vec<u8>,
        prepackmsg_buffer: Vec<u8>,
        // the randomize digits, generated on the C++ side(using abseil)
        digits: Vec<u8>,
    }

    unsafe extern "C++" {
        include!("lib-garble-wrapper/src/rust_wrapper.h");

        type GarbleWrapper;

        fn new_garble_wrapper() -> UniquePtr<GarbleWrapper>;

        fn GarbleSkcdFromBuffer(&self, skcd_buffer: Vec<u8>) -> Vec<u8>;
        fn GarbleAndStrippedSkcdFromBuffer(&self, skcd_buffer: Vec<u8>) -> StrippedCircuit;
        fn PackmsgFromPrepacket(&self, prepackmsg_buffer: &Vec<u8>, message: String) -> Vec<u8>;
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
