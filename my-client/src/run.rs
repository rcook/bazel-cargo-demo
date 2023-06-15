use crate::args::Args;
use anyhow::Result;
use clap::Parser;
use my_lib::adder::adder_client::AdderClient;
use my_lib::adder::AddNumbersRequest;
use tonic::Request;
use tracing::instrument;

#[instrument]
pub async fn run() -> Result<()> {
    let args = Args::parse();
    let mut client = AdderClient::connect(args.endpoint).await?;
    let response = client
        .add_numbers(Request::new(AddNumbersRequest { a: 5, b: 6 }))
        .await?;
    println!("sum={}", response.into_inner().sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {}
}
