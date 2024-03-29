use crate::hash;
use crate::repo::Repo;
use rust_code_analysis::{get_function_spaces, read_file, CodeMetrics, FuncSpace, SpaceKind, LANG};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FileData {
    pub name: String,
    pub path: String,
    pub extension: String,
    pub language: String,
    // Contains data about the code from rust_code_analysis
    // We need to skip_deserializing here because `FuncSpace` does not implement
    // Deserialize
    #[serde(skip_deserializing)]
    // skip_deserializing does not work well on the `spaces` field because
    // `FuncSpace` does not implement `Default`, forcing us set a default
    #[serde(default = "default_func_spaces")]
    pub spaces: FuncSpace,
}

/// This function will be called when the default value of `spaces` is needed
pub fn default_func_spaces() -> FuncSpace {
    FuncSpace {
        name: Some("".to_string()),
        start_line: 0,
        end_line: 0,
        kind: SpaceKind::Function,
        spaces: Vec::new(),
        metrics: CodeMetrics::default(),
    }
}

impl Default for FileData {
    fn default() -> FileData {
        FileData {
            name: Default::default(),
            path: Default::default(),
            extension: Default::default(),
            language: Default::default(),
            spaces: FuncSpace {
                name: Default::default(),
                start_line: Default::default(),
                end_line: Default::default(),
                kind: SpaceKind::Unknown,
                spaces: Default::default(),
                metrics: Default::default(),
            },
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Code {
    pub repo_name: String,
    pub files_data: HashMap<String, FileData>,
}

pub fn get_repo_path(repo: &Repo) -> Result<&Path, String> {
    match repo.repo.path().parent() {
        Some(repo_path) => Ok(repo_path),
        None => Err(format!(
            "Failed to get repo path, repo: {:?}",
            repo.repo.path().to_str()
        )),
    }
}

pub fn get_file_language(file_extension: &String) -> Option<LANG> {
    match file_extension.as_ref() {
        "js" => Some(LANG::Javascript),
        "jsm" => Some(LANG::Mozjs),
        "java" => Some(LANG::Java),
        "rs" => Some(LANG::Rust),
        "cpp" | "cxx" | "cc" | "hxx" | "hpp" | "c" | "h" | "hh" | "inc" | "mm" | "m" => {
            Some(LANG::Cpp)
        }
        "py" => Some(LANG::Python),
        "tsx" => Some(LANG::Tsx),
        "ts" | "jsw" | "jsmw" => Some(LANG::Typescript),
        _ => None,
    }
}

pub fn extract_code_data(repo_path: &Path) -> Result<Code, String> {
    let repo_name = match repo_path.file_name() {
        Some(repo_name) => String::from(repo_name.to_string_lossy()),
        None => {
            return Err(format!(
                "Couldn't get repo name, path: {}",
                repo_path.display()
            ))
        }
    };

    let mut code_data = Code {
        repo_name,
        files_data: HashMap::new(),
    };

    // Extract code data from each file in the given repository
    // if the file type is supported by rust_code_analysis
    for file in WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_path = file.path();
        let file_name = match file_path.file_name() {
            Some(file_name) => String::from(file_name.to_string_lossy()),
            None => continue,
        };

        let file_extension = match file_path.extension() {
            Some(file_extension) => String::from(file_extension.to_string_lossy()),
            None => continue,
        };

        let file_language = match get_file_language(&file_extension) {
            Some(file_language) => file_language,
            None => continue,
        };

        let source_code = match read_file(file_path) {
            Ok(source_code) => source_code,
            Err(_) => continue,
        };

        let spaces = match get_function_spaces(&file_language, source_code, file_path, None) {
            Some(functions_spaces) => functions_spaces,
            None => continue,
        };

        //Get the path file relative to the root of the git repository
        //for a /Users/elhmn/.wake/scanner/github-com-elhmn-qautomata/src/example.rs file
        //the `path` will be `src/example.rs`
        let path = match file_path.strip_prefix(String::from(repo_path.to_string_lossy())) {
            Ok(p) => p.to_str().unwrap_or_default().to_string(),
            Err(_) => "".to_string(),
        };

        let file_data = FileData {
            name: file_name,
            path: path.clone(),
            extension: file_extension,
            language: String::from(file_language.get_name()),
            spaces,
        };

        code_data.files_data.insert(hash::new(path), file_data);
    }

    Ok(code_data)
}

pub fn new(repo: &Repo) -> Result<Code, String> {
    let repo_path = get_repo_path(repo)?;
    let code_data = extract_code_data(repo_path)?;

    Ok(code_data)
}
