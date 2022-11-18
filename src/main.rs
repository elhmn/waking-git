mod config;
mod cmd;
mod repo;
mod extractor;

fn main() {
    let conf = config::Config::new();
    cmd::run(conf);
}
