use axum::{routing::get, Router};

pub fn run(port: String) {
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
        .route("/scan", get(get_scan))
        .route("/extracted", get(get_extracted))
        .route("/converted", get(get_converted));

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//This example should pretty much show you how to write basic handler
//with status code and response https://github.com/tokio-rs/axum/blob/main/examples/todos/src/main.rs
async fn get_scan() -> &'static str {
    "Get scan"
}

async fn get_extracted() -> &'static str {
    "Get extracted"
}

async fn get_converted() -> &'static str {
    "Get converted"
}
