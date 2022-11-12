use clap::Args;

#[derive(Args, Debug)]
pub struct RunCmd {}

pub fn run(_cmd: &RunCmd) {
    println!("scanner called");
}
