use crate::repo;

pub struct Git {
    pub test: String,
}

pub fn new(repo: Option<repo::Repo>) -> Git {
    println!("new git extractor called !");

    Git {
        test: "repo".to_string(),
    }
}
