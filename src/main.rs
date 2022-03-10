use clap::Parser;
use tonic::transport::Server;

mod garble_routes;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Where to reach the IPFS node
    #[clap(long, default_value = "/ip4/127.0.0.1/tcp/5001")]
    ipfs_server_multiaddr: String,
}

// TODO DRY server creation with the tests
// cf https://github.com/hyperium/tonic/blob/4b0ece6d2854af088fbc1bdb55c2cdd19ec9bb92/tonic-web/tests/integration/tests/grpc.rs#L113
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    // TODO configurable port
    let addr = "0.0.0.0:3000".parse().unwrap();

    let circuits_api = garble_routes::GarbleApiServerImpl {
        ipfs_server_multiaddr: args.ipfs_server_multiaddr,
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

    Server::builder()
        .accept_http1(true)
        .add_service(circuits_api)
        .serve(addr)
        .await?;

    Ok(())
}
