use crate::config;
use clap::Args;
use core::server;

#[derive(Args, Debug)]
pub struct RunArgs {
    #[clap(short, long)]
    /// specify the port you want the serve to listen on
    port: Option<String>,
}

pub fn run(args: &RunArgs, _conf: config::Config) {
    let port = args.port.to_owned().unwrap_or_else(|| "8080".to_string());

    server::run(port);
}
