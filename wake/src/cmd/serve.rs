use crate::config;
use clap::Args;

#[derive(Args, Debug)]
pub struct RunArgs {
    #[clap(short, long)]
    /// specify the port you want the serve to listen on
    port: Option<String>,
}

pub fn run(args: &RunArgs, _conf: config::Config) {
    let port = args.port.as_deref().unwrap_or("8080");

    println!("Running on port {}", port);
}
