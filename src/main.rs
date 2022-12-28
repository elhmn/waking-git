mod cmd;
mod config;
mod converters;
mod extractor;
mod hash;
mod players;
mod repo;

fn main() {
    let conf = config::Config::new();
    cmd::run(conf);
}
