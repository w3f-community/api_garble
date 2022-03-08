// https://github.com/hyperium/tonic/issues/727
// https://github.com/hyperium/tonic/blob/master/tests/integration_tests/tests/timeout.rs

// TODO? use integration_tests::pb::{test_client, test_server, Input, Output};
// use ipfs_embed::{Config, DefaultParams, Ipfs};
use bytes::Buf;
use bytes::BufMut;
use prost::Message;
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;
use tonic::{transport::Server, Code, Request, Response, Status};
use ipfs_api_backend_hyper::TryFromUri;
use ipfs_api_backend_hyper::IpfsApi;
use std::io::Cursor;

// mod garble_routes;
use api_garble::garble_routes::{self, interstellarpbapigarble::GarbleIpfsReply};

mod foreign_ipfs;

pub mod interstellarpbapigarble {
    tonic::include_proto!("interstellarpbapigarble");
}

// we CAN NOT just send the raw encoded protobuf(eg using GarbleIpfsRequest{}.encode())
// b/c that returns errors like
// "protocol error: received message with invalid compression flag: 8 (valid flags are 0 and 1), while sending request"
// "tonic-web: Invalid byte 45, offset 0"
// https://github.com/hyperium/tonic/blob/01e5be508051eebf19c233d48b57797a17331383/tonic-web/tests/integration/tests/grpc_web.rs#L93
// also: https://github.com/grpc/grpc-web/issues/152
fn encode_body(skcd_cid: &str) -> bytes::Bytes {
    let input = interstellarpbapigarble::GarbleIpfsRequest {
        skcd_cid: skcd_cid.to_string(),
    };

    let mut buf = bytes::BytesMut::with_capacity(1024);
    buf.reserve(5);
    unsafe {
        buf.advance_mut(5);
    }

    input.encode(&mut buf).unwrap();

    let len = buf.len() - 5;
    {
        let mut buf = &mut buf[..5];
        buf.put_u8(0);
        buf.put_u32(len as u32);
    }

    buf.split_to(len + 5).freeze()
}

async fn decode_body(body: hyper::Body, content_type: &str) -> (GarbleIpfsReply, bytes::Bytes) {
    let mut body = hyper::body::to_bytes(body).await.unwrap();

    if content_type == "application/grpc-web-text+proto" {
        body = base64::decode(body).unwrap().into()
    }

    body.advance(1);
    let len = body.get_u32();
    let msg = GarbleIpfsReply::decode(&mut body.split_to(len as usize)).expect("decode");
    body.advance(5);

    (msg, body)
}

#[tokio::test]
async fn endpoint_garble_ipfs_grpc_web() {
    let foreign_node = run_ipfs_in_background().await;
    let ipfs_server_multiaddr = format!("/ip4/127.0.0.1/tcp/{}", foreign_node.api_port);
    let addr = run_service_in_background(
        Duration::from_secs(1),
        Duration::from_secs(100),
        &ipfs_server_multiaddr,
    )
    .await;

    // read a .skcd test file
    let skcd_data = std::fs::read_to_string("./tests/data/adder.skcd.pb.bin").unwrap();
    // let verilog_data = std::fs::read("./tests/data/adder.v").unwrap();

    // insert a basic .skcd in IPFS
    let ipfs_client =
        ipfs_api_backend_hyper::IpfsClient::from_multiaddr_str(&ipfs_server_multiaddr).unwrap();
    let skcd_cursor = Cursor::new(skcd_data);
    // "ApiError { message: "Invalid byte while expecting start of value: 0x2f", code: 0 }"
    // let ipfs_result = ipfs_client.dag_put(skcd_cursor).await.unwrap();
    let ipfs_result = ipfs_client.add(skcd_cursor).await.unwrap();

    let request_uri = format!(
        "http://{}/interstellarpbapigarble.GarbleApi/GarbleIpfs",
        addr
    );

    let client = hyper::Client::new();

    let body_buf = encode_body(&ipfs_result.hash);

    let content_type = "application/grpc-web";
    let accept = "application/grpc-web";
    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .header(hyper::header::CONTENT_TYPE, content_type)
        // .header(hyper::header::ORIGIN, "http://example.com")
        .header(hyper::header::ACCEPT, accept)
        .uri(request_uri)
        .body(hyper::Body::from(body_buf))
        .unwrap();

    let res = client.request(req).await.unwrap();

    assert_eq!(res.status(), hyper::StatusCode::OK);
    let (reply, trailers) = decode_body(res.into_body(), content_type).await;
    assert_eq!(
        reply.pgarbled_cid.len(),
        "Qmf1rtki74jvYmGeqaaV51hzeiaa6DyWc98fzDiuPatzyy".len()
    );
    assert_eq!(&trailers[..], b"grpc-status:0\r\n");
}


async fn run_service_in_background(
    latency: Duration,
    server_timeout: Duration,
    ipfs_server_multiaddr: &str,
) -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let circuits_api = garble_routes::GarbleApiServerImpl {
        ipfs_server_multiaddr: ipfs_server_multiaddr.to_string(),
    };
    let circuits_api =
        garble_routes::interstellarpbapigarble::garble_api_server::GarbleApiServer::new(
            circuits_api,
        );
    // let greeter = InterstellarCircuitsApiClient::new(greeter);
    let circuits_api = tonic_web::config()
        .allow_origins(vec!["127.0.0.1"])
        .enable(circuits_api);

    println!("GreeterServer listening on {}", addr);

    tokio::spawn(async move {
        Server::builder()
            .accept_http1(true)
            .add_service(circuits_api)
            // .serve(addr) // NO!
            // thread 'cancelation_on_timeout' panicked at 'called `Result::unwrap()` on an `Err`
            // value: tonic::transport::Error(Transport, hyper::Error(Connect, ConnectError("tcp connect error",
            // Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })))',
            // tests/circuit_gen_endpoint_test.rs:24:6
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
            .await
            .unwrap();
    });

    addr
}

// https://github.com/ipfs-rust/ipfs-embed/#getting-started
async fn run_ipfs_in_background() -> foreign_ipfs::ForeignNode {
    // https://github.com/rs-ipfs/rust-ipfs/blob/master/tests/pubsub.rs
    let foreign_node = foreign_ipfs::ForeignNode::new();
    let foreign_api_port = foreign_node.api_port;
    println!("run_ipfs_in_background: port: {}", foreign_api_port);

    // MUST be returned and kept alive; else the daemon is killed
    foreign_node

    // ALTERNATIVE: https://docs.ipfs.io/install/ipfs-desktop/#ubuntu
}
