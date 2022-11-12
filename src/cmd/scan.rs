use clap::Args;

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the path to the repository we want to scan
    #[clap(value_name = "REPOSITORY", index = 1)]
    repos: Option<String>
}

pub fn run(args: &RunArgs) {
    let repos = args.repos.clone().unwrap_or("".to_string());
    println!("scanner called: {:?}", repos);
}
