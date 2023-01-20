pub mod shmup;

use crate::extractor;

pub trait Converter<Data> {
    /// run the converter
    fn run(&self, extracted_data: &extractor::Data) -> Result<Data, String>;

    /// Return converter name
    fn name(&self) -> String {
        "default".to_string()
    }
}
