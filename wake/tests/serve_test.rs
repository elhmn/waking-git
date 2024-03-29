use assert_cmd::prelude::*;
use core::utils::test;
use core::utils::test::TMP_DIR;
use core::{converters::shmup, extractor, server};

use std::process::{Child, Command};

const SERVER_PORT: &str = "4242";

fn start_server() -> Child {
    let mut cmd = Command::cargo_bin("wake").unwrap();
    let port = std::env::var("SERVER_PORT").unwrap_or(SERVER_PORT.to_string());
    let child = cmd
        .current_dir(TMP_DIR)
        .arg("serve")
        .arg("--port")
        .arg(port)
        .spawn()
        .unwrap();

    while !is_server_running() {
        println!("Server not running");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("Server running");
    child
}

fn is_server_running() -> bool {
    let url = "http://localhost:4242/ping".to_string();
    let resp = match reqwest::blocking::get(url) {
        Ok(resp) => resp,
        Err(err) => {
            println!("Failed to run server: {}", err);
            return false;
        }
    };

    resp.status().is_success()
}

#[test]
fn get_scan_extracted() -> Result<(), Box<dyn std::error::Error>> {
    test::setup();
    let mut child = start_server();

    //test that /scan/extracted with correct body returns 200
    {
        let body = server::ScanRequest {
            repo_url: Some("https://github.com/elhmn/ckp".to_string()),
            _ref_: Some("".to_string()),
        };

        let json_body = serde_json::to_string(&body).unwrap();
        let url = "http://localhost:4242/scan/extracted".to_string();
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .header("content-type", "application/json")
            .body(json_body)
            .send()
            .unwrap();

        assert!(resp.status().is_success());
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/json"
        );
        //Deserialise the response body in a extractor::Data struct
        let body = resp.text().unwrap();
        //Should panic if the response body is wrong
        let _data: extractor::Data = serde_json::from_str(&body).unwrap();
    }

    //test that /scan/extracted with wrong body returns 500
    {
        let body = server::ScanRequest {
            repo_url: Some("https://wrong_url".to_string()),
            _ref_: Some("".to_string()),
        };

        let json_body = serde_json::to_string(&body).unwrap();
        let url = "http://localhost:4242/scan/extracted".to_string();
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .header("content-type", "application/json")
            .body(json_body)
            .send()
            .unwrap();

        assert!(resp.status().is_server_error());
    }

    //stop the server
    child.kill().unwrap();
    test::teardown();
    Ok(())
}

#[test]
fn get_scan_converted() -> Result<(), Box<dyn std::error::Error>> {
    test::setup();
    let mut child = start_server();

    //test that /scan/converted with correct body returns 200
    {
        let body = server::ScanRequest {
            repo_url: Some("https://github.com/elhmn/ckp".to_string()),
            _ref_: Some("".to_string()),
        };

        let json_body = serde_json::to_string(&body).unwrap();
        let url = "http://localhost:4242/scan/converted".to_string();
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .header("content-type", "application/json")
            .body(json_body)
            .send()
            .unwrap();

        assert!(resp.status().is_success());
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/json"
        );
        //Deserialise the response body in a convertor::Data struct
        let body = resp.text().unwrap();
        //Should panic if the response body is wrong
        let _data: shmup::Data = serde_json::from_str(&body).unwrap();
    }

    //test that /scan/extracted with wrong body returns 500
    {
        let body = server::ScanRequest {
            repo_url: Some("https://wrong_url".to_string()),
            _ref_: Some("".to_string()),
        };

        let json_body = serde_json::to_string(&body).unwrap();
        let url = "http://localhost:4242/scan/converted".to_string();
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .header("content-type", "application/json")
            .body(json_body)
            .send()
            .unwrap();

        assert!(resp.status().is_server_error());
    }

    //stop the server
    child.kill().unwrap();
    test::teardown();
    Ok(())
}

#[test]
fn get_scan() -> Result<(), Box<dyn std::error::Error>> {
    test::setup();
    let mut child = start_server();

    //test that /scan with correct body returns 200
    {
        let body = server::ScanRequest {
            repo_url: Some("https://github.com/elhmn/ckp".to_string()),
            _ref_: Some("".to_string()),
        };

        let json_body = serde_json::to_string(&body).unwrap();
        let url = "http://localhost:4242/scan".to_string();
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .header("content-type", "application/json")
            .body(json_body)
            .send()
            .unwrap();

        assert!(resp.status().is_success());
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/gzip"
        );
        assert_eq!(resp.headers().get("content-encoding").unwrap(), "gzip");
        //assert header content-disposition
        assert_eq!(
            resp.headers().get("content-disposition").unwrap(),
            "attachment; filename=github-com-elhmn-ckp.tar.gz"
        );
    }

    //test that /scan with wrong body returns 500
    {
        let body = server::ScanRequest {
            repo_url: Some("https://wrong_url".to_string()),
            _ref_: Some("".to_string()),
        };

        let json_body = serde_json::to_string(&body).unwrap();
        let url = "http://localhost:4242/scan".to_string();
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .header("content-type", "application/json")
            .body(json_body)
            .send()
            .unwrap();

        assert!(resp.status().is_server_error());
    }

    //stop the server
    child.kill().unwrap();
    test::teardown();
    Ok(())
}
