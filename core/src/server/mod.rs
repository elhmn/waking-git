use crate::repo;
use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Deserialize;
use simple_logger;
use std::fs::File;
use flate2::Compression;
use tar::Builder;

pub fn run(port: String) {
    simple_logger::init().unwrap();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            serve(port).await;
        })
}

async fn serve(port: String) {
    println!("Server running on port {port}");

    // build our application with a single route
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/scan", get(get_scan))
        .route("/scan/extracted", get(get_extracted))
        .route("/scan/converted", get(get_converted));

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct ScanRequest {
    repo_url: Option<String>,
    //TODO: add ref
    _ref_: Option<String>,
}

//This example should pretty much show you how to write basic handler
//with status code and response https://github.com/tokio-rs/axum/blob/main/examples/todos/src/main.rs
async fn get_scan(Json(payload): Json<ScanRequest>) -> (StatusCode, String) {
    let repo = payload.repo_url.unwrap_or_default();
    let conf = crate::config::Config::new();
    let (_, converted, repo) = match scan(conf, repo) {
        Ok(d) => d,
        Err(err) => {
            log::error!("Failed to scan data: {err}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to scan data".to_owned(),
            );
        }
    };

    let file = File::create("tmp.tar").unwrap();
    let mut a = Builder::new(file);

    //     a.append_path(repo.extracted_file_path).unwrap();
    a.append_file(
        "converted.json".to_owned(),
        &mut File::open(repo.converted_file_path.to_owned()).unwrap(),
    )
    .unwrap();
    a.append_file(
        "extracted.json".to_owned(),
        &mut File::open(repo.extracted_file_path.to_owned()).unwrap(),
    )
    .unwrap();

    //TODO: send back tarball of scanned data
    //the tarball should contain 2 files, extracted.json and converted.json
    (StatusCode::OK, converted)
}

async fn get_extracted(Json(payload): Json<ScanRequest>) -> (StatusCode, String) {
    let repo = payload.repo_url.unwrap_or_default();
    let conf = crate::config::Config::new();
    let (extracted, _, _) = match scan(conf, repo) {
        Ok(d) => d,
        Err(err) => {
            log::error!("Failed to extract data: {err}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to extract data".to_owned(),
            );
        }
    };

    (StatusCode::OK, extracted)
}

async fn get_converted(Json(payload): Json<ScanRequest>) -> (StatusCode, String) {
    let repo = payload.repo_url.unwrap_or_default();
    let conf = crate::config::Config::new();
    let (_, converted, _) = match scan(conf, repo) {
        Ok(d) => d,
        Err(err) => {
            log::error!("Failed to convert data: {err}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to convert data".to_owned(),
            );
        }
    };

    (StatusCode::OK, converted)
}

async fn ping() -> &'static str {
    "pong"
}

fn scan(conf: crate::config::Config, repo: String) -> Result<(String, String, repo::Repo), String> {
    let mut git_repo = match crate::repo::clone_repository(&repo, &conf) {
        Ok(r) => r,
        Err(err) => {
            return Err(format!("while cloning repository: {err}"));
        }
    };

    let (extracted_data, extracted_json_data) =
        match crate::extractor::extract(&conf, &mut git_repo) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!("failed to extract repository data: {err}"));
            }
        };

    let conv = crate::converters::shmup::new();
    let (_, converted_json_data) =
        match crate::converters::convert(&conf, &mut git_repo, extracted_data, &conv) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!("failed to convert extracted data: {err}"));
            }
        };

    Ok((extracted_json_data, converted_json_data, git_repo))
}
