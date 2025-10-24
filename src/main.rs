mod config;
mod route;

use crate::config::Config;
use std::{
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    path::Path,
};

use actix_web::{App, HttpServer};
use anyhow::Context;
use clap::Parser;
use simple_logger::SimpleLogger;

#[derive(clap::Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long, help = "Path of config file")]
    config: String,
    #[arg(long, default_value_t = false, help = "Set logger to debug mode")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    init_logger(args.verbose)?;
    let config = Config::try_from(Path::new(&args.config))?;
    let listener = init_listener(config.listen_port)?;

    HttpServer::new(move || App::new().configure(route::config))
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

fn init_listener(port: u16) -> anyhow::Result<TcpListener> {
    let listening_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    let listener =
        TcpListener::bind(listening_addr).context("Failed to start listening at given address")?;

    Ok(listener)
}
