use crate::repo;

//Metrics contains data extracted from `git-sizer`
pub struct Metrics {}

pub struct Git {
    //     pub metrics: Metrics,
    test: String,
}

pub fn new(repo: &repo::Repo) -> Result<Git, String> {
    println!("new git extractor called !");
    Ok(Git {
        test: "repo".to_string(),
    })
}
