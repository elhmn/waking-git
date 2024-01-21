use crate::{converters, extractor, languages, shapes};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct ShmupConverter {}

const CONVERTER_NAME: &str = "shmup";

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Data {
    //the id of the main scene
    //it corresponds to the root tree of the git repository
    pub main_scene: String,
    pub scenes: HashMap<String, Scene>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Scene {
    pub entities: HashMap<String, Entity>,
    pub sub_scenes: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Entity {
    pub id: String,
    //the scene id the object belongs to
    pub scene_id: String,
    pub name: String,
    //the kind is the shape the entity will take
    //the values supported are circle | triangle | hexagon | triangle
    pub kind: String,
    pub color: String,
    pub weapon: String,
    pub movement_pattern: String,
    pub speed: f32,
    //the hp is a value between [0-1]
    pub hp: f32,
    //the size is a value between [0-1]
    pub size: f32,
    pub shield: String,
    pub destructible: bool,
}

pub fn new() -> ShmupConverter {
    ShmupConverter {}
}

impl converters::Converter<Data> for ShmupConverter {
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

    let data = Data {
        main_scene: get_main_scene(extracted_data),
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

fn get_main_scene(data: &extractor::Data) -> String {
    let root_commit_id = &data.git.ref_target.1;
    if let Some(root_commit) = &data.git.objects[root_commit_id].commit {
        return root_commit.tree.to_string();
    }

    "".to_string()
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

            // Create entities
            for oid in &tree.objects {
                if let Some(blob) = &objs[oid].blob {
                    let mut entity = blob_to_entity(blob, files);
                    entity.scene_id = oid.to_owned();
                    scene.entities.insert(oid.to_owned(), entity);
                } else {
                    scene.sub_scenes.push(oid.to_owned());
                }
            }

            if !scene.sub_scenes.is_empty() {
                add_scenes(&scene.sub_scenes, data.to_owned(), objs, files);
            }

            data.lock()
                .unwrap()
                .scenes
                .insert(tree.sha.to_owned(), scene);
        }
    });
}

fn blob_to_entity(
    blob: &extractor::git::Blob,
    files: &HashMap<String, extractor::code::FileData>,
) -> Entity {
    let languages = languages::new();
    Entity {
        id: blob.sha.to_owned(),
        name: blob.name.to_owned(),
        color: get_color(blob, &languages),
        kind: get_kind(blob, &languages),
        speed: get_speed(blob, files),
        hp: 1.,
        ..Default::default()
    }
}

/// This function returns the speed of an entity
/// relative its spaces
///
/// It's important to note that this function is
/// a placeholder and is only used to demonstrate
/// how we can map files with blob data
fn get_speed(
    blob: &extractor::git::Blob,
    files: &HashMap<String, extractor::code::FileData>,
) -> f32 {
    match files.get(&blob.path_sha) {
        Some(file) => {
            let ret = 1. / file.spaces.spaces.len() as f32;
            if ret.is_infinite() {
                1.
            } else {
                ret
            }
        }
        None => 0.1,
    }
}

/// determine the color of the entity from its
/// extension
fn get_color(blob: &extractor::git::Blob, languages: &languages::Languages) -> String {
    let p = Path::new(blob.path.as_str());
    if let Some(ext) = p.extension() {
        let converted_extension = format!(".{}", ext.to_string_lossy().into_owned());
        return languages::color_from_extension(languages, &converted_extension);
    }

    "".to_owned()
}

/// returns the kind of language file the blob is
fn get_kind(blob: &extractor::git::Blob, languages: &languages::Languages) -> String {
    let p = Path::new(blob.path.as_str());
    let mut kind = "".to_owned();
    if let Some(ext) = p.extension() {
        let converted_extension = format!(".{}", ext.to_string_lossy().into_owned());
        kind = languages::kind_from_extension(languages, &converted_extension);
    }

    kind_to_shape(kind.as_str()).to_owned()
}

/// convert a kind to a known shape that will be used by the
/// player
fn kind_to_shape(kind: &str) -> &str {
    match kind {
        "data" => shapes::RECTANGLE,
        "prose" => shapes::RECTANGLE,
        "markup" => shapes::HEXAGON,
        "programming" => shapes::CIRCLE,
        _ => shapes::TRIANGLE,
    }
}
