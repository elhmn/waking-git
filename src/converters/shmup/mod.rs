use crate::{converters, extractor};
use serde::Serialize;
use std::collections::HashMap;

pub struct ShmupConverter {}

const CONVERTER_NAME: &str = "shmup";

#[derive(Serialize, Default)]
pub struct Data {
    pub scenes: HashMap<String, Scene>,
}

#[derive(Serialize, Default)]
pub struct Scene {
    pub entities: HashMap<String, Entity>,
    pub sub_scenes: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct Entity {
    pub id: String,
    //scene id of the object belongs to
    pub scene_id: String,
    pub name: String,
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

    let mut data = Data {
        ..Default::default()
    };

    //Get the initial commit
    if let Some(commit) = &objs[commit_oid].commit {
        let trees_oid = vec![commit.tree.clone()];
        add_scenes(&trees_oid, &mut data, objs);
    }

    data
}

fn add_scenes(
    trees_oid: &Vec<String>,
    data: &mut Data,
    objs: &HashMap<String, extractor::git::Object>,
) {
    for tree_oid in trees_oid {
        if let Some(tree) = &objs[tree_oid].tree {
            let mut scene = Scene {
                ..Default::default()
            };

            // Create entities
            for oid in &tree.objects {
                if let Some(blob) = &objs[oid].blob {
                    let mut entity = blob_to_entity(blob);
                    entity.scene_id = oid.clone();
                    scene.entities.insert(oid.clone(), entity);
                } else {
                    scene.sub_scenes.push(oid.clone());
                }
            }

            if !scene.sub_scenes.is_empty() {
                add_scenes(&scene.sub_scenes, data, objs);
            }

            data.scenes.insert(tree.sha.clone(), scene);
        }
    }
}

fn blob_to_entity(blob: &extractor::git::Blob) -> Entity {
    Entity {
        id: blob.sha.clone(),
        name: blob.name.clone(),
        color: "a color".to_string(),
        kind: "kind".to_string(),
        speed: 0.2,
        hp: 1.,
        ..Default::default()
    }
}
