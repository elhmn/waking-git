use crate::{converters, extractor, languages};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct CodeAlkemiConverter {}

const CONVERTER_NAME: &str = "codealkemi";

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Data {
    pub version: String,
    pub name: String,
    pub repo_name: String,
    pub url: String,
    pub commit: String,
    //The id of the main scene
    //it corresponds to the root tree of the git repository
    //and is stored as the sha256 of that path.
    pub main_scene: String,
    pub scenes: HashMap<String, Scene>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Scene {
    // Entities, has the sha256 of the git blob as its key, and the entity
    // as value.
    pub entities: HashMap<String, Entity>,
    pub sub_scenes: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Language {
    pub name: String,
    pub kind: String,
    pub color: String,
    pub extension: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Entity {
    //The id is the Sha256 of the the blob path
    pub id: String,
    // the oid, is the git object id
    pub oid: String,
    //the scene id the object belongs to
    pub scene_id: String,
    pub name: String,
    pub path: String,
    pub language: Language,
}

pub fn new() -> CodeAlkemiConverter {
    CodeAlkemiConverter {}
}

impl converters::Converter<Data> for CodeAlkemiConverter {
    fn run(&self, extracted_data: &extractor::Data) -> Result<Data, String> {
        Ok(build_converter_data(extracted_data))
    }

    fn name(&self) -> String {
        CONVERTER_NAME.to_owned()
    }
}

fn build_converter_data(extracted_data: &extractor::Data) -> Data {
    let commit_oid = &extracted_data.git.ref_target.1;
    let objs = &extracted_data.git.objects;
    let files = &extracted_data.code.files_data;
    let repo_name = &extracted_data.code.repo_name;

    let data = Data {
        version: "v0".to_owned(),
        main_scene: get_main_scene(extracted_data),
        commit: commit_oid.to_string(),
        repo_name: repo_name.to_owned(),
        name: CONVERTER_NAME.to_owned(),
        ..Default::default()
    };

    let mut_data = Arc::new(Mutex::new(data));

    //Get the initial commit
    if let Some(commit) = &objs[commit_oid].commit {
        let trees_oid = vec![commit.tree.to_owned()];
        add_scenes(&trees_oid, mut_data.to_owned(), objs, files);
    }

    let data = mut_data.lock().unwrap().to_owned();
    data
}

// Gets sha256 path of the root directory.
// This is used as the main scene of the game, and it will be used to traverse the rest of the
// scenes.
fn get_main_scene(data: &extractor::Data) -> String {
    let root_commit_id = &data.git.ref_target.1;

    let Some(commit) = &data.git.objects[root_commit_id].commit else {
        return String::new();
    };

    let Some(tree) = &data.git.objects[&commit.tree].tree else {
        return String::new();
    };

    tree.path_sha.clone()
}

fn add_scenes(
    trees_oid: &Vec<String>,
    data: Arc<Mutex<Data>>,
    objs: &HashMap<String, extractor::git::Object>,
    files: &HashMap<String, extractor::code::FileData>,
) {
    trees_oid.par_iter().for_each(|tree_oid| {
        if let Some(tree) = &objs[tree_oid].tree {
            let mut scene = Scene {
                ..Default::default()
            };
            let mut sub_scene_oids: Vec<String> = vec![];

            // Create entities
            for oid in &tree.objects {
                if let Some(blob) = &objs[oid].blob {
                    let mut entity = blob_to_entity(blob, files);
                    entity.scene_id = oid.to_owned();
                    scene.entities.insert(blob.path_sha.to_owned(), entity);
                } else {
                    // Store sub_scenes path_sha
                    if let Some(tree) = &objs[oid].tree {
                        scene.sub_scenes.push(tree.path_sha.to_owned());
                    }

                    sub_scene_oids.push(oid.to_owned());
                }
            }

            if !sub_scene_oids.is_empty() {
                add_scenes(&sub_scene_oids, data.to_owned(), objs, files);
            }

            data.lock()
                .unwrap()
                .scenes
                .insert(tree.path_sha.to_owned(), scene);
        }
    });
}

fn blob_to_entity(
    blob: &extractor::git::Blob,
    _files: &HashMap<String, extractor::code::FileData>,
) -> Entity {
    let languages = languages::new();
    let spec = get_language_spec(blob, &languages);
    let language = Language {
        color: spec.color,
        kind: spec.kind,
        name: spec.name,
        ..Default::default()
    };

    Entity {
        id: blob.path_sha.to_owned(),
        oid: blob.sha.to_owned(),
        name: blob.name.to_owned(),
        path: blob.path.to_owned(),
        language: language,
        ..Default::default()
    }
}

fn get_language_spec(
    blob: &extractor::git::Blob,
    languages: &languages::Languages,
) -> languages::Spec {
    let p = Path::new(blob.path.as_str());
    if let Some(ext) = p.extension() {
        let converted_extension = format!(".{}", ext.to_string_lossy().into_owned());
        return languages::spec_from_extension(languages, &converted_extension);
    }

    return languages::Spec {
        ..Default::default()
    };
}
