pub mod shmup;
use crate::config;
use crate::extractor;
use crate::repo;
use crate::utils;

pub trait Converter<Data> {
    /// run the converter
    fn run(&self, extracted_data: &extractor::Data) -> Result<Data, String>;

    /// Return converter name
    fn name(&self) -> String {
        "default".to_owned()
    }
}

pub fn convert<Data: serde::Serialize>(
    conf: &config::Config,
    git_repo: &mut repo::Repo,
    extracted_data: extractor::Data,
    converter: &impl Converter<Data>,
) -> Result<(Data, String), String> {
    let data = converter.run(&extracted_data)?;
    let dest_folder = format!(
        "{}/{}/{}",
        conf.wake_path,
        config::SCANNER_FOLDER_NAME,
        git_repo.folder_name
    );
    let dest_path = format!(
        "{}/{}-{}",
        dest_folder,
        converter.name(),
        config::CONVERTER_FILE_NAME_PREFIX
    );
    git_repo.converted_file_path = dest_path.clone();
    let json_data = serde_json::to_string(&data).unwrap_or_else(|_| "".to_string());
    match utils::store_json_data(json_data.to_owned(), dest_folder, &dest_path) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Error: failed to convert repository data: {err}"));
        }
    };

    Ok((data, json_data))
}
