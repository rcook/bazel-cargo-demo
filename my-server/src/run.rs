use crate::args::Args;
use crate::service::Service;
use anyhow::Result;
use clap::Parser;
use my_lib::adder::adder_server::AdderServer;
use tonic::transport::Server;
use tracing::{info, instrument};

#[instrument]
pub async fn run() -> Result<()> {
    let args = Args::parse();
    info!("Server listening on {}", args.address);
    Server::builder()
        .add_service(AdderServer::new(Service::default()))
        .serve(args.address)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {}
}
