#![doc = include_str!("../README.md")]

use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use tracing::info;

use self::args::Args;

mod args;
mod log;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    log::setup_tracing();

    for _ in 0..args.count {
        info!(name=?args.name);
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tokio::test;

    #[test]
    async fn dummy() -> Result<()> {
        log::setup_tracing();
        tracing::warn!("No actual unit tests yet");
        assert_eq!(4, 3+1);
        Ok(())
    }
}
