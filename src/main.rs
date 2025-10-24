use std::net::{SocketAddrV4, TcpListener};

use actix_web::{App, HttpServer};
use anyhow::Context;
use clap::Parser;
use simple_logger::SimpleLogger;

#[derive(clap::Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = false, help = "Set logger to debug mode")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    init_logger(args.verbose)?;

    let listening_addr = "0.0.0.0:8080";
    let listening_addr: SocketAddrV4 = listening_addr
        .parse()
        .context("Failed to parse listening addr")?;
    let listener =
        TcpListener::bind(listening_addr).context("Failed to start listening at given address")?;

    HttpServer::new(move || App::new())
        .listen(listener)
        .context("Http server cannot start listening")?
        .run()
        .await
        .context("Http server is crash")?;

    Ok(())
}

fn init_logger(verbose: bool) -> anyhow::Result<()> {
    let log_level = if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    SimpleLogger::new()
        .with_level(log_level)
        .init()
        .context("Failed to initiate logger")?;

    Ok(())
}
