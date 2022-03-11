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

fn main() {
    tonic_build::configure()
        .build_server(true)
        // TODO build_client only needed for tests
        .build_client(true)
        .compile(
            // list of protos
            &[
                "deps/protos/api_garble/api.proto",
                "deps/protos/api_garble/garble_routes.proto",
            ],
            // includes
            &["deps/protos"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
