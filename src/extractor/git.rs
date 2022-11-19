use crate::repo;
use serde::Serialize;
use git2;
use std::collections::HashMap;

#[derive(Serialize, Default)]
pub enum ObjectKind {
    Blob,
    Tree,
    Commit,
    Tags,
    Ref,
    #[default]
    Unknown,
}

//Metrics contains data extracted from `git-sizer`
//we can run `git-sizer --json` on the repository`
//and use the output to populate the metrics
//it is a little bit more involved and can be done
//a later on
#[derive(Serialize, Default)]
pub struct Metrics {}

#[derive(Serialize, Default)]
pub struct Blob {
    pub permissions: String,
    pub name: String,
    pub sha: String,
}

#[derive(Serialize, Default)]
pub struct Tree {
    pub name: String,
    //sha the git object hash
    pub sha: String,
    pub permissions: String,
    //objects contain a list of git objects hash
    pub objects: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct Tag {
    pub name: String,
    pub message: String,
    //kind is equivalent to the `type`
    pub kind: ObjectKind,
    pub tagger: String,
    pub sha: String,
    //object is the sha of the object the tagged is applied to
    pub object: String,
}

#[derive(Serialize, Default)]
pub struct Commit {
    pub name: String,
    //sha the git object hash
    pub sha: String,
    pub author: String,
    pub committer: String,
    pub message: String,
    pub tree: String,
    //parents is a list of parent commits
    //only merge commits may have more than one parent
    pub parents: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct Object {
    pub kind: ObjectKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<Blob>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree: Option<Tree>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<Commit>,
}

#[derive(Serialize, Default)]
pub struct Git {
    pub metrics: Metrics,
    pub objects: HashMap<String, Object>,
    pub refs: HashMap<String, String>,
}

pub fn new(repo: &repo::Repo) -> Result<Git, String> {
    let git_data = match extrat_git_objects(&repo) {
        Ok(d) => d,
       Err(err) => {
           return Err(format!("failed to extract git objects: {}", err).to_string())
       }
    };

    println!("{}", serde_json::to_string(&git_data).unwrap()); // Debug
    Ok(git_data)
}

pub fn extrat_git_objects(repo: &repo::Repo) -> Result<Git, git2::Error> {
    let r = &repo.repo;

    //Get all repository references
    let mut walk = r.revwalk()?;
    let mut oid: git2::Oid;
    for rf in repo.repo.references()? {
        //add each reference objects in the walker
        if let Some(ref_name) = rf.unwrap().name() {
            oid = r.revparse_single(ref_name)?.id();
            println!("ref_name: [{}]", ref_name);
            walk.push(oid)?;
        }
    }

    for w in walk {
        println!("[{}]", w.unwrap().to_string())
    }

    Ok(Git {
        objects: HashMap::from([(
            "sha".to_string(),
            Object {
                ..Default::default()
            })]),
        ..Default::default()
    })
}
