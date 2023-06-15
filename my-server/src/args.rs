use std::net::SocketAddr;

use clap::Parser;

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {}
}
