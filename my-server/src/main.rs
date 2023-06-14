mod args;
mod service;

use crate::args::Args;
use crate::service::Service;
use anyhow::Result;
use clap::Parser;
use my_lib::hello_world::greeter_server::GreeterServer;
use tonic::transport::Server;
use tracing::{info, instrument};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<()> {
    fmt()
        .compact()
        .with_span_events(FmtSpan::NEW | FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        .init();
    run().await
}

#[instrument]
async fn run() -> Result<()> {
    let args = Args::parse();
    let service = Service::default();

    info!("Server listening on {}", args.address);

    Server::builder()
        .add_service(GreeterServer::new(service))
        .serve(args.address)
        .await?;

    Ok(())
}
