mod cmd;
mod config;
mod extractor;
mod repo;

fn main() {
    let conf = config::Config::new();
    cmd::run(conf);
}
