use crate::repo;
use axum::{
    body::StreamBody,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use simple_logger;
use std::fs::File;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use tar::Builder;
use tokio_util::io::ReaderStream;

struct Server {
    pub port: String,
    pub tx: Mutex<Sender<Arc<Task>>>,
    pub rx: Mutex<Receiver<Arc<Task>>>,
}

type ScanResult = (String, String, repo::Repo);

struct Task {
    pub task: fn(conf: Arc<crate::config::Config>, repo: Arc<String>) -> Result<ScanResult, String>,
    pub repo: Arc<String>,
    pub conf: Arc<crate::config::Config>,
    //TODO: create type for the Sender and Receiver,
    //it is quite complicated to read at the moment
    pub tx: Mutex<Sender<Result<ScanResult, String>>>,
    pub rx: Mutex<Receiver<Result<ScanResult, String>>>,
}

impl Task {
    pub fn new(
        task: fn(conf: Arc<crate::config::Config>, repo: Arc<String>) -> Result<ScanResult, String>,
        repo: Arc<String>,
        conf: Arc<crate::config::Config>,
    ) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            task,
            repo,
            conf,
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
        }
    }
}

pub fn run(port: String) {
    //Initialise the verbose logger
    //TODO: it should be a little less verbose
    simple_logger::init().unwrap();

    let (tx, rx) = mpsc::channel();
    let server: Arc<Server> = Arc::new(Server {
        port,
        tx: Mutex::new(tx),
        rx: Mutex::new(rx),
    });

    //Start the task scheduler.
    //
    //This is a temporary solution, it only allow us to spin up
    //long running scan operations concurrently without
    //prioritizing short running operations. Meaning requests
    //on small repositories will end up sharing the same thread pool
    //as requests made on big repositories as `kubernetes/kubernetes` and `torvalds/linux`.
    //A single long running request can slow down all other requests.
    //
    //We need to find a way to prioritize short running requests, maybe by running
    //the long running tasks in a separate thread pool or in separate processes.
    //
    //The current solution is good enough for now, but it is not ideal.
    let s = server.clone();
    rayon::spawn(move || {
        while let Ok(task) = s.rx.lock().unwrap().recv() {
            rayon::spawn(move || {
                let t = task.clone();
                let ret = t.task.to_owned()(t.conf.to_owned(), t.repo.to_owned());
                task.tx.lock().unwrap().send(ret).unwrap();
            })
        }
    });

    let main_runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("tokio-main")
        .enable_all()
        .build()
        .unwrap();
    main_runtime.block_on(async {
        serve(server.clone()).await;
    });
}

async fn serve(server: Arc<Server>) {
    println!("Server running on port {}", server.port);

    // build our application with a single route
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/scan", get(get_scan))
        .route("/scan/extracted", get(get_extracted))
        .route("/scan/converted", get(get_converted))
        .layer(Extension(server.clone()));

    axum::Server::bind(&format!("0.0.0.0:{}", server.port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Serialize)]
pub struct ScanRequest {
    pub repo_url: Option<String>,
    //TODO: add ref
    pub _ref_: Option<String>,
}

//This example should pretty much show you how to write basic handler
//with status code and response https://github.com/tokio-rs/axum/blob/main/examples/todos/src/main.rs
async fn get_scan(
    server: Extension<Arc<Server>>,
    Json(payload): Json<ScanRequest>,
) -> impl IntoResponse {
    let repo = payload.repo_url.unwrap_or_default();
    let conf = crate::config::Config::new();
    let task = Arc::new(Task::new(scan, Arc::new(repo), Arc::new(conf)));

    //Sending the task to the scheduler
    if let Err(err) = server.tx.lock().unwrap().send(task.clone()) {
        log::error!("Failed to send task: {}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to send task".to_owned(),
        ));
    };

    //Wait for the scheduler response
    let (_, _, repo) = match task.rx.lock().unwrap().recv().unwrap() {
        Ok(d) => d,
        Err(err) => {
            log::error!("Failed to scan data: {err}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to scan data".to_owned(),
            ));
        }
    };

    //Create a compressed tarball
    let tarball = format!("{}/{}.tar", &repo.scanner_path, &repo.folder_name);
    let compressed_tarball = format!("{tarball}.gz");
    let output_file = File::create(&compressed_tarball).unwrap();
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let mut builder = Builder::new(&mut encoder);
    builder
        .append_file(
            "converted.json",
            &mut File::open(&repo.converted_file_path).unwrap(),
        )
        .unwrap();
    builder
        .append_file(
            "extracted.json",
            &mut File::open(&repo.extracted_file_path).unwrap(),
        )
        .unwrap();
    builder.finish().unwrap();

    //Sending the compressed tarball to the client
    let file = match tokio::fs::File::open(&compressed_tarball).await {
        Ok(file) => file,
        Err(err) => {
            log::error!("Failed to open file: {err}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to open file".to_owned(),
            ));
        }
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    //Setting response headers
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/gzip".parse().unwrap());
    headers.insert(header::CONTENT_ENCODING, "gzip".parse().unwrap());
    //TODO: add a proper content length header
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!(
            "attachment; filename={}",
            format_args!("{}.tar.gz", repo.folder_name)
        )
        .parse()
        .unwrap(),
    );

    Ok((StatusCode::OK, headers, body))
}

async fn get_extracted(
    server: Extension<Arc<Server>>,
    Json(payload): Json<ScanRequest>,
) -> impl IntoResponse {
    let repo = payload.repo_url.unwrap_or_default();
    let conf = crate::config::Config::new();
    let task = Arc::new(Task::new(scan, Arc::new(repo), Arc::new(conf)));

    //Sending the task to the scheduler
    if let Err(err) = server.tx.lock().unwrap().send(task.clone()) {
        log::error!("Failed to send task: {}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to send task".to_owned(),
        ));
    }

    //Wait for the scheduler response
    let (extracted, _, _) = match task.rx.lock().unwrap().recv().unwrap() {
        Ok(d) => d,
        Err(err) => {
            log::error!("Failed to extract data: {err}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to extract data".to_owned(),
            ));
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    Ok((StatusCode::OK, headers, extracted))
}

async fn get_converted(
    server: Extension<Arc<Server>>,
    Json(payload): Json<ScanRequest>,
) -> impl IntoResponse {
    let repo = payload.repo_url.unwrap_or_default();
    let conf = crate::config::Config::new();
    let task = Arc::new(Task::new(scan, Arc::new(repo), Arc::new(conf)));

    //Sending the task to the scheduler
    if let Err(err) = server.tx.lock().unwrap().send(task.clone()) {
        log::error!("Failed to send task: {}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to send task".to_owned(),
        ));
    }

    //Wait for the scheduler response
    let ret = task.rx.lock().unwrap().recv().unwrap();
    let (_, converted, _) = match ret {
        Ok(d) => d,
        Err(err) => {
            log::error!("Failed to convert data: {err}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to convert data".to_owned(),
            ));
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    Ok((StatusCode::OK, headers, converted))
}

async fn ping() -> &'static str {
    "pong"
}

fn scan(conf: Arc<crate::config::Config>, repo: Arc<String>) -> Result<ScanResult, String> {
    let mut git_repo = match crate::repo::clone_repository(&repo, &conf) {
        Ok(r) => r,
        Err(err) => {
            return Err(format!("while cloning repository: {err}"));
        }
    };

    let (extracted_data, extracted_json_data) = match crate::extractor::extract(&mut git_repo) {
        Ok(d) => d,
        Err(err) => {
            return Err(format!("failed to extract repository data: {err}"));
        }
    };

    let conv = crate::converters::shmup::new();
    let (_, converted_json_data) =
        match crate::converters::convert(&mut git_repo, extracted_data, &conv) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!("failed to convert extracted data: {err}"));
            }
        };

    Ok((extracted_json_data, converted_json_data, git_repo))
}
