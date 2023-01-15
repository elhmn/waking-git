use crate::hash;
use crate::repo;
use git2::{self, Repository, TreeEntry};
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
    //path is the path to the file/directory relative
    //to the root tree
    //this is used to map git data to code data.
    //that hashed path is used as a key to a code entry
    pub path: String,
    //path_sha is the sha 256 of the path.
    //this is used to optimize code data lookup
    pub path_sha: String,
    pub sha: String,
}

#[derive(Serialize, Default)]
pub struct Tree {
    pub name: String,
    //path is the path to the file/directory relative
    //to the root tree.
    //this is used to map git data to code data.
    //that hashed path is used as a key to a code entry
    pub path: String,
    //path_sha is the sha 256 of the path.
    //this is used to optimize code data lookup
    pub path_sha: String,
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
    //the ref_target is the ref used to traverse git the
    //repository tree.
    // (String, String) <=> (ref, oid)
    pub ref_target: (String, String),
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
    let git_data = match extract_git_objects(repo) {
        Ok(d) => d,
        Err(err) => return Err(format!("failed to extract git objects: {}", err)),
    };

    Ok(git_data)
}

pub fn extract_git_objects(repo: &repo::Repo) -> Result<Git, git2::Error> {
    let r = &repo.repo;

    let mut ref_name = "refs/heads/master";
    //Get default reference oid
    //First we check for `master` and if `master` does not exist we fallback to `main`
    let oid = match repo.repo.refname_to_id(ref_name) {
        Ok(oid) => oid,
        Err(_) => {
            ref_name = "refs/heads/main";
            repo.repo.refname_to_id(ref_name)?
        }
    };
    let mut objects: HashMap<String, Object> = HashMap::new();
    //For each commit found from the references
    let mut obj = Object::new();
    let commit = r.find_commit(oid)?;
    obj.kind = ObjectKind::Commit;
    //Get single commit
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

    Ok(Git {
        objects,
        ref_target: (ref_name.to_string(), format!("{}", oid)),
        ..Default::default()
    })
}

fn add_tree_objects(
    tree: &git2::Tree,
    objects: &mut HashMap<String, Object>,
    repo: &git2::Repository,
) -> Result<(), git2::Error> {
    //Create the root tree object
    {
        let mut obj = Object::new();
        obj.kind = ObjectKind::Tree;
        obj.tree = Some(Tree {
            name: "".to_string(),
            path: "".to_string(),
            path_sha: hash::new("".to_string()),
            sha: tree.id().to_string(),
            filemode: 0,
            objects: tree.iter().map(|t| t.id().to_string()).collect(),
        });

        objects.insert(tree.id().to_string(), obj);
    }

    tree.walk(git2::TreeWalkMode::PreOrder, |path, entry| {
        let mut obj = Object::new();
        if let Some(kind) = entry.kind() {
            match kind {
                //Create and add Tree objects
                git2::ObjectType::Tree => {
                    obj.kind = ObjectKind::Tree;
                    obj.tree = Some(build_tree_object(path.to_string(), entry, repo));
                }

                //Create and add Blob objects
                git2::ObjectType::Blob => {
                    let name = entry.name().unwrap_or("").to_string();
                    let path = get_relative_path(path.to_string(), name.clone());
                    obj.kind = ObjectKind::Blob;
                    obj.blob = Some(Blob {
                        name,
                        path: path.clone(),
                        path_sha: hash::new(path),
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

fn build_tree_object<'tree>(path: String, entry: &TreeEntry<'tree>, repo: &Repository) -> Tree {
    let name = entry.name().unwrap_or("").to_string();
    let path = get_relative_path(path, name.clone());
    Tree {
        name,
        sha: entry.id().to_string(),
        path: path.clone(),
        path_sha: hash::new(path),
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
    }
}

pub fn get_relative_path(path: String, file_name: String) -> String {
    format!("{}{}", path, file_name)
}

#[cfg(test)]
mod tests {
    use crate::extractor::git;

    #[test]
    fn test_get_relative_path() {
        assert_eq!(
            git::get_relative_path("src/".to_string(), "test.rs".to_string()),
            "src/test.rs"
        );
        assert_eq!(
            git::get_relative_path("src/".to_string(), "".to_string()),
            "src/"
        );

        assert_eq!(
            git::get_relative_path("".to_string(), "test".to_string()),
            "test"
        );
    }
}
