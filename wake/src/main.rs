mod cmd;
use core::config;

fn main() {
    let conf = config::Config::new();
    cmd::run(conf);
}
