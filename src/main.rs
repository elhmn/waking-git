mod config;
mod cmd;
mod repo;

fn main() {
    let conf = config::Config::new();
    cmd::run(conf);
}
