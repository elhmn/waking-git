use crate::repo;
use git2;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Default)]
pub enum ObjectKind {
    Blob,
    Tree,
    Commit,
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
    pub filemode: i32,
    pub name: String,
    pub sha: String,
}

#[derive(Serialize, Default)]
pub struct Tree {
    pub name: String,
    //sha the git object hash
    pub sha: String,
    pub filemode: i32,
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
    //commit_sha is the sha of the commit object the tagged is applied to
    pub commit_sha: String,
}

#[derive(Serialize, Default)]
pub struct Commit {
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

impl Object {
    fn new() -> Object {
        Object {
            ..Default::default()
        }
    }
}

pub fn new(repo: &repo::Repo) -> Result<Git, String> {
    let git_data = match extrat_git_objects(repo) {
        Ok(d) => d,
        Err(err) => return Err(format!("failed to extract git objects: {}", err)),
    };

    Ok(git_data)
}

pub fn extrat_git_objects(repo: &repo::Repo) -> Result<Git, git2::Error> {
    let r = &repo.repo;

    //Get default reference oid
    //First we check for `master` and if `master` does not exist we fallback to `main`
    let oid = match repo.repo.refname_to_id("refs/heads/master") {
        Ok(oid) => oid,
        Err(_) => repo.repo.refname_to_id("refs/heads/main")?,
    };

    let mut walk = r.revwalk()?;
    walk.push(oid)?;

    let mut objects: HashMap<String, Object> = HashMap::new();
    //For each commit found from the references
    for w in walk {
        let mut obj = Object::new();
        let oid = w?;
        let commit = r.find_commit(oid)?;

        obj.kind = ObjectKind::Commit;

        //Get commits
        obj.commit = Some(Commit {
            author: commit.author().to_string(),
            sha: commit.id().to_string(),
            message: commit.message().unwrap_or("").to_string(),
            tree: commit.tree()?.id().to_string(),
            committer: commit.committer().to_string(),
            parents: {
                let mut ids = vec![];
                for id in commit.parent_ids() {
                    ids.push(id.to_string());
                }
                ids
            },
        });
        //Add the commit object in the objects HashMap
        objects.insert(oid.to_string(), obj);

        //Add every git objects found during the tree object traversal
        add_tree_objects(&commit.tree()?, &mut objects, r)?;
    }

    Ok(Git {
        objects,
        ..Default::default()
    })
}

fn add_tree_objects(
    tree: &git2::Tree,
    objects: &mut HashMap<String, Object>,
    repo: &git2::Repository,
) -> Result<(), git2::Error> {
    tree.walk(git2::TreeWalkMode::PreOrder, |_, entry| {
        let mut obj = Object::new();

        if let Some(kind) = entry.kind() {
            match kind {
                //Create and add Tree objects
                git2::ObjectType::Tree => {
                    obj.kind = ObjectKind::Tree;
                    obj.tree = Some(Tree {
                        name: entry.name().unwrap_or("").to_string(),
                        sha: entry.id().to_string(),
                        filemode: entry.filemode(),
                        objects: (|| -> Vec<String> {
                            let mut objs = vec![];
                            let t = repo.find_tree(entry.id()).unwrap();

                            //We walk down the tree to find every blob or tree objects and add them
                            //to our list of objects
                            t.walk(git2::TreeWalkMode::PreOrder, |_, entry| {
                                objs.push(entry.id().to_string());
                                if Some(git2::ObjectType::Tree) == entry.kind() {
                                    return git2::TreeWalkResult::Skip;
                                }
                                git2::TreeWalkResult::Ok
                            })
                            .unwrap();
                            objs
                        })(),
                    });
                }

                //Create and add Blob objects
                git2::ObjectType::Blob => {
                    obj.kind = ObjectKind::Blob;
                    obj.blob = Some(Blob {
                        name: entry.name().unwrap_or("").to_string(),
                        sha: entry.id().to_string(),
                        filemode: entry.filemode(),
                    });
                }
                _ => (),
            }
        };

        objects.insert(entry.id().to_string(), obj);
        git2::TreeWalkResult::Ok
    })?;

    Ok(())
}
