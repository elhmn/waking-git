mod config;
mod cmd;

fn main() {
    let conf = config::Config::new();
    cmd::run(conf);
}
