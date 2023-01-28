use crate::{converters, extractor, languages, shapes};
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

pub struct ShmupConverter {}

const CONVERTER_NAME: &str = "shmup";

#[derive(Serialize, Default, Debug)]
pub struct Data {
    pub scenes: HashMap<String, Scene>,
}

#[derive(Serialize, Default, Debug)]
pub struct Scene {
    pub entities: HashMap<String, Entity>,
    pub sub_scenes: Vec<String>,
}

#[derive(Serialize, Default, Debug)]
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
        CONVERTER_NAME.to_string()
    }
}

fn build_converter_data(extracted_data: &extractor::Data) -> Data {
    let commit_oid = &extracted_data.git.ref_target.1;
    let objs = &extracted_data.git.objects;
    let files = &extracted_data.code.files_data;

    let mut data = Data {
        ..Default::default()
    };

    //Get the initial commit
    if let Some(commit) = &objs[commit_oid].commit {
        let trees_oid = vec![commit.tree.clone()];
        add_scenes(&trees_oid, &mut data, objs, files);
    }

    data
}

fn add_scenes(
    trees_oid: &Vec<String>,
    data: &mut Data,
    objs: &HashMap<String, extractor::git::Object>,
    files: &HashMap<String, extractor::code::FileData>,
) {
    for tree_oid in trees_oid {
        if let Some(tree) = &objs[tree_oid].tree {
            let mut scene = Scene {
                ..Default::default()
            };

            // Create entities
            for oid in &tree.objects {
                if let Some(blob) = &objs[oid].blob {
                    let mut entity = blob_to_entity(blob, files);
                    entity.scene_id = oid.clone();
                    scene.entities.insert(oid.clone(), entity);
                } else {
                    scene.sub_scenes.push(oid.clone());
                }
            }

            if !scene.sub_scenes.is_empty() {
                add_scenes(&scene.sub_scenes, data, objs, files);
            }

            data.scenes.insert(tree.sha.clone(), scene);
        }
    }
}

fn blob_to_entity(
    blob: &extractor::git::Blob,
    files: &HashMap<String, extractor::code::FileData>,
) -> Entity {
    let languages = languages::new();
    Entity {
        id: blob.sha.clone(),
        name: blob.name.clone(),
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
        Some(file) => 1. / file.spaces.spaces.len() as f32,
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

    "".to_string()
}

/// returns the kind of language file the blob is
fn get_kind(blob: &extractor::git::Blob, languages: &languages::Languages) -> String {
    let p = Path::new(blob.path.as_str());
    let mut kind = "".to_string();
    if let Some(ext) = p.extension() {
        let converted_extension = format!(".{}", ext.to_string_lossy().into_owned());
        kind = languages::kind_from_extension(languages, &converted_extension);
    }

    kind_to_shape(kind.as_str()).to_string()
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