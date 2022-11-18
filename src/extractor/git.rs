use crate::repo;

//Metrics contains data extracted from `git-sizer`
pub struct Metrics {}

pub struct Git {
    //     pub metrics: Metrics,
    test: String,
}

pub fn new(repo: repo::Repo) -> Git {
    println!("new git extractor called !");
    Git {
        test: "repo".to_string(),
    }
}
