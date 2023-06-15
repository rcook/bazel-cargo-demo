mod args;
mod run;
mod service;

use crate::run::run;
use anyhow::Result;
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

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {}
}
