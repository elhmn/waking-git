use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Language {
    fs_name: Option<String>,
    #[serde(rename = "type")]
    kind: String,
    aliases: Option<Vec<String>>,
    code_mirror_mode: Option<String>,
    code_mirror_mime_type: Option<String>,
    wrap: Option<bool>,
    extensions: Option<Vec<String>>,
    filenames: Option<Vec<String>>,
    interpreters: Option<Vec<String>>,
    language_id: i32,
    color: Option<String>,
    tm_scope: Option<String>,
    group: Option<String>,
}

type Languages = BTreeMap<String, Language>;

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
