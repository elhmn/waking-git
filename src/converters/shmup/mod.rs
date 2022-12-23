use crate::{converters, extractor};
use serde::Serialize;

pub struct ShmupConverter {}

const CONVERTER_NAME: &str = "shmup";

#[derive(Serialize, Default)]
pub struct Data {
    yolo: String,
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
