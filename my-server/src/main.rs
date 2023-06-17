mod args;
mod run;
mod service;

use crate::args::Args;
use crate::run::run;
use anyhow::Result;
use clap::Parser;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::{fmt, layer};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let span_events = FmtSpan::NEW | FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE;

    let args = Args::parse();
    if args.debug {
        tracing_subscriber::registry()
            .with(console_subscriber::spawn())
            .with(layer().compact().with_span_events(span_events))
            .init();
    } else {
        fmt().compact().with_span_events(span_events).init();
    }

    run(args.address).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {}
}
