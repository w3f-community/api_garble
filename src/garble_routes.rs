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

use ipfs_api_backend_hyper::{
    BackendWithGlobalOptions, Error, GlobalOptions, IpfsApi, IpfsClient, TryFromUri,
};
// use ipfs_embed::{Config, DefaultParams, Ipfs};
use futures_util::TryStreamExt;
use log;
use std::fs::File;
use std::io::Cursor;
use std::io::{Read, Seek, SeekFrom, Write};
use std::time::Duration;
use tempfile::Builder;
use tonic::{Request, Response, Status};

use interstellarpbapigarble::garble_api_server::GarbleApi;
use interstellarpbapigarble::garble_api_server::GarbleApiServer;
use interstellarpbapigarble::{GarbleIpfsReply, GarbleIpfsRequest};

use lib_garble_wrapper::cxx::UniquePtr;
use lib_garble_wrapper::ffi;
use lib_garble_wrapper::ffi::GarbleWrapper;

pub mod interstellarpbapigarble {
    tonic::include_proto!("interstellarpbapigarble");
}

// #[derive(Default)]
pub struct GarbleApiServerImpl {
    pub ipfs_server_multiaddr: String,
}

trait HasIpfsClient {
    fn ipfs_client(&self) -> BackendWithGlobalOptions<IpfsClient>;
}

impl HasIpfsClient for GarbleApiServerImpl {
    fn ipfs_client(&self) -> BackendWithGlobalOptions<IpfsClient> {
        log::info!(
            "ipfs_client: starting with: {}",
            &self.ipfs_server_multiaddr
        );
        BackendWithGlobalOptions::new(
            ipfs_api_backend_hyper::IpfsClient::from_multiaddr_str(&self.ipfs_server_multiaddr)
                .unwrap(),
            GlobalOptions::builder()
                .timeout(Duration::from_millis(5000))
                .build(),
        )
    }
}

#[tonic::async_trait]
impl GarbleApi for GarbleApiServerImpl {
    async fn garble_ipfs(
        &self,
        request: Request<GarbleIpfsRequest>,
    ) -> Result<Response<GarbleIpfsReply>, Status> {
        log::info!("Got a request from {:?}", request.remote_addr());
        let skcd_cid = &request.get_ref().skcd_cid;

        // get the (.skcd) from IPFS
        // DO NOT use dag_get if the file was "add"
        // The returned bytes would be eg
        // {"Data":{"/":{"bytes":"CAISjgQvL....ZfYWRkGI4E"}},"Links":[]}
        // let skcd_buf = self
        //     .ipfs_client()
        //     .dag_get(&skcd_cid)
        //     .map_ok(|chunk| chunk.to_vec())
        //     .try_concat()
        //     .await
        //     .unwrap();
        let skcd_buf = self
            .ipfs_client()
            .cat(&skcd_cid)
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await
            .unwrap();

        let tmp_dir = Builder::new()
            .prefix("interstellar-garble_routes-garble_ipfs")
            .tempdir()
            .unwrap();

        // write the data from IPFS to a temp file
        let skcd_input_path = tmp_dir.path().join("input.skcd.pb.bin");
        std::fs::write(&skcd_input_path, skcd_buf).expect("could not write to skcd_input_path");

        // TODO class member/Trait for "lib_garble_wrapper::ffi::new_garble_wrapper()"
        let lib_garble_wrapper = tokio::task::spawn_blocking(move || {
            let wrapper = lib_garble_wrapper::ffi::new_garble_wrapper();

            // TODO make the C++ API return a buffer?
            let buf: Vec<u8> =
                wrapper.GarbleSkcdToBuffer(skcd_input_path.as_os_str().to_str().unwrap());

            buf
        })
        .await
        .unwrap();

        let data = Cursor::new(lib_garble_wrapper);

        // TODO error handling, or at least logging
        let ipfs_result = self.ipfs_client().add(data).await.unwrap();

        let reply = GarbleIpfsReply {
            pgarbled_cid: format!("{}", ipfs_result.hash),
        };

        Ok(Response::new(reply))
    }
}
