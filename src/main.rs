mod cmd;
mod config;
mod converters;
mod extractor;
mod hash;
mod languages;
mod players;
mod repo;
mod shapes;

fn main() {
    let conf = config::Config::new();
    cmd::run(conf);
}
