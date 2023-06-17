use clap::Parser;
use std::net::SocketAddr;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(
        help = "Address",
        short = 'a',
        long = "address",
        default_value = "[::1]:50051",
        env = "MY_SERVER_ADDRESS"
    )]
    pub address: SocketAddr,

    #[clap(
        help = "Debug mode using tokio-console",
        short = 'd',
        long = "debug",
        default_value = "false",
        env = "MY_SERVER_DEBUG"
    )]
    pub debug: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {}
}
