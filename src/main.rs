use std::net::SocketAddr;
mod configuration;
// mod tidb_service;
pub mod utils;
pub mod kafka;
// pub mod tidb {
//     tonic::include_proto!("tidb");
// }
// use crate::tidb_service::service::TidbService;
// use tidb::tidb_server::TidbServer;
// use tonic::transport::Server;
use utils::logger::startLogger;

#[tokio::main]
async fn main() -> Result<(), String> {
    startLogger();
    // let addr = SocketAddr::from(([0, 0, 0, 0], configuration::get("port")));
    // let tidb_service = TidbService::default();
    // let res = Server::builder()
    //     .add_service(TidbServer::new(tidb_service))
    //     .serve(addr)
    //     .await;
    // match res {
    //     Ok(_res) => {
    //         println!("Server Started On {:?}", addr);
    //     }
    //     Err(error) => {
    //         println!("Error While Starting Grpc Server{}", error);
    //     }
    // }
    Ok(())
}
