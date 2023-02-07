use crate::repo;
use axum::{http::StatusCode, routing::get, Extension, Json, Router};
use serde::Deserialize;
use simple_logger;
use std::fs::File;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use tar::Builder;

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
        .route("/scan/extracted", get(get_extracted))
        .route("/scan", get(get_scan))
        .route("/scan/converted", get(get_converted))
        .layer(Extension(server.clone()));

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{}", server.port).parse().unwrap())
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
    let (_, converted, repo) = match scan(Arc::new(conf), Arc::new(repo)) {
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
        "converted.json",
        &mut File::open(&repo.converted_file_path).unwrap(),
    )
    .unwrap();
    a.append_file(
        "extracted.json",
        &mut File::open(&repo.extracted_file_path).unwrap(),
    )
    .unwrap();

    //TODO: send back tarball of scanned data
    //the tarball should contain 2 files, extracted.json and converted.json
    (StatusCode::OK, converted)
}

async fn get_extracted(
    server: Extension<Arc<Server>>,
    Json(payload): Json<ScanRequest>,
) -> (StatusCode, String) {
    let repo = payload.repo_url.unwrap_or_default();
    let conf = crate::config::Config::new();

    let (tx, rx) = mpsc::channel();

    //Initialising the task
    let task = Task {
        task: scan,
        tx: Mutex::new(tx),
        rx: Mutex::new(rx),
        conf: Arc::new(conf),
        repo: Arc::new(repo),
    };
    let task = Arc::new(task);

    //Sending the task to the scheduler
    if let Err(err) = server.tx.lock().unwrap().send(task.clone()) {
        log::error!("Failed to send task: {}", err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to send task".to_owned(),
        );
    }

    //Wait for the scheduler response
    let ret = task.rx.lock().unwrap().recv().unwrap();

    let (extracted, _, _) = match ret {
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

async fn get_converted(
    server: Extension<Arc<Server>>,
    Json(payload): Json<ScanRequest>,
) -> (StatusCode, String) {
    let repo = payload.repo_url.unwrap_or_default();
    let conf = crate::config::Config::new();

    let (tx, rx) = mpsc::channel();

    //Initialising the task
    let task = Task {
        task: scan,
        tx: Mutex::new(tx),
        rx: Mutex::new(rx),
        conf: Arc::new(conf),
        repo: Arc::new(repo),
    };
    let task = Arc::new(task);

    //Sending the task to the scheduler
    if let Err(err) = server.tx.lock().unwrap().send(task.clone()) {
        log::error!("Failed to send task: {}", err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to send task".to_owned(),
        );
    }

    //Wait for the scheduler response
    let ret = task.rx.lock().unwrap().recv().unwrap();
    let (_, converted, _) = match ret {
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

fn scan(conf: Arc<crate::config::Config>, repo: Arc<String>) -> Result<ScanResult, String> {
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
