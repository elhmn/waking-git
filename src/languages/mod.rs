use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Language {
    pub fs_name: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    pub aliases: Option<Vec<String>>,
    pub code_mirror_mode: Option<String>,
    pub code_mirror_mime_type: Option<String>,
    pub wrap: Option<bool>,
    pub extensions: Option<Vec<String>>,
    pub filenames: Option<Vec<String>>,
    pub interpreters: Option<Vec<String>>,
    pub language_id: i32,
    pub color: Option<String>,
    pub tm_scope: Option<String>,
    pub group: Option<String>,
}

pub type Languages = BTreeMap<String, Language>;

pub fn color_from_extension(languages: &Languages, extension: &str) -> String {
    let mut found = Vec::new();

    for l in languages.values() {
        if let Some(e) = &l.extensions {
            if e.contains(&extension.to_string()) {
                found.push(l);
            }
        }
    }
    let mut color = "".to_string();

    if !found.is_empty() {
        let mut min = i32::MAX;
        for f in found {
            // we exclude file that have no tm_scope set as that means
            // we have no grammar supported for these language entry
            if f.language_id < min && f.tm_scope.clone().unwrap_or_default() != "none" {
                color = f.color.clone().unwrap_or_default();
                min = f.language_id;
            }
        }
    }

    color
}

/// To use cautiously as it loads the entire language file every time
/// it is called
pub fn new() -> Languages {
    //The ./res/languages.yml file will be embeded in the binary
    //
    //the file is extracted from
    //https://github.com/github/linguist/blob/master/lib/linguist/languages.yml
    // and should be regularly updated
    let bytes = include_bytes!("./res/languages.yml");
    let yaml = String::from_utf8_lossy(bytes);

    // Deserialize it back to a Rust type.
    serde_yaml::from_str(yaml.into_owned().as_str()).unwrap()
}
