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
    pub sub_scenes: HashMap<String, Scene>,
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
    fn run(&self, _extracted_data: &extractor::Data) -> Result<Data, String> {
        println!("ShmupConverter called!");
        Ok(Data {
            ..Default::default()
        })
    }

    fn name(&self) -> String {
        CONVERTER_NAME.to_string()
    }
}
