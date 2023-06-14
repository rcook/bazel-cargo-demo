use std::net::SocketAddr;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(
        help = "Socket address",
        short = 'a',
        long = "address",
        default_value = "[::1]:50051",
        env = "MY_SERVER_SOCKET_ADDRESS"
    )]
    pub address: SocketAddr,
}
