use lib::cmd;
use lib::config;

fn main() {
    let conf = config::Config::new();
    cmd::run(conf);
}
