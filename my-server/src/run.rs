use crate::service::Service;
use anyhow::Result;
use my_lib::adder::adder_server::AdderServer;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::{info, instrument};

#[instrument]
pub async fn run(address: SocketAddr) -> Result<()> {
    info!("Server listening on {}", address);
    Server::builder()
        .add_service(AdderServer::new(Service::default()))
        .serve(address)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {}
}
