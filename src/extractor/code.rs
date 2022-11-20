use crate::repo::Repo;
use rust_code_analysis::{
    dump_root, metrics, read_file, JavascriptParser, ParserTrait, RustParser,
};
use std::path::Path;
use walkdir::WalkDir;
use serde::Serialize;

static ALLOWED_FILE_EXTENSIONS: [&str; 10] = [
    ".cpp", ".cs", ".css", ".go", ".html", ".java", ".js", ".py", ".rs", ".ts",
];

#[derive(Serialize, Default)]
pub struct Code {
    string: String,
}

pub fn get_repo_path(repo: &Repo) -> Result<&Path, String> {
    match repo.repo.path().parent() {
        Some(repo_path) => return Ok(repo_path),
        None => {
            return Err(format!(
                "Failed to get repo path, repo: {:?}",
                repo.repo.path().to_str()
            ));
        }
    }
}

pub fn filter_file_extension(file: &walkdir::DirEntry) -> bool {
    let file_name = String::from(file.file_name().to_string_lossy());

    for file_extension in ALLOWED_FILE_EXTENSIONS {
        if file_name.ends_with(file_extension) {
            return true;
        }
    }

    false
}

pub fn find_all_files_in_repo(repo_path: &Path) -> Result<String, String> {
    for file in WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| filter_file_extension(e))
    {
        println!("{}", file.path().display());

        let source_code = match read_file(file.path()) {
            Ok(source_code) => source_code,
            Err(_) => continue,
        };

        let file_extension = match file.path().extension() {
            Some(file_extension) => file_extension,
            None => continue,
        };

        match file_extension.to_string_lossy().as_ref() {
            "js" => {
                let parser = JavascriptParser::new(source_code, file.path(), None);
                let space = metrics(&parser, file.path()).unwrap();
                dump_root(&space).unwrap();
            }
            "rs" => {
                let parser = RustParser::new(source_code, file.path(), None);
                let space = metrics(&parser, file.path()).unwrap();
                dump_root(&space).unwrap();
            }
            _ => continue,
        };
    }

    Ok("TMP".to_string())
}

pub fn new(repo: &Repo) -> Result<Code, String> {
    let repo_path = get_repo_path(repo)?;

    find_all_files_in_repo(repo_path);

    Ok(Code {
        string: "TMP".to_string(),
    })
}
