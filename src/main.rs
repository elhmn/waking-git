mod cmd;
mod config;

fn main() {
    let conf = config::Config::new();
    cmd::run(conf);
}
