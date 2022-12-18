use crate::config;
use crate::players::shmup;
use clap::Args;

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the path to the directory to use to generate
    /// the world.
    /// example:
    /// `wake play -d ./tmp/github-com-elhmn-ckp`
    #[arg(short, long, value_name = "DIRECTORY")]
    dir: Option<String>,
}

pub fn run(args: &RunArgs, _conf: config::Config) {
    let dir = args.dir.clone().unwrap_or_default();
    println!("Play run command invoked");
    shmup::run();
    if !dir.is_empty() {
        println!("Called with {}", dir);
    }
}
