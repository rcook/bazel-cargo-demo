use clap::Parser;
use tonic::transport::Endpoint;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(
        help = "Endpoint",
        short = 'e',
        long = "endpoint",
        default_value = "http://[::1]:50051",
        env = "MY_CLIENT_ENDPOINT"
    )]
    pub endpoint: Endpoint,
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {}
}
