#![cfg(test)]

use actix_web::{http, test, HttpResponse};
use super::handle;
use std::fs;
use actix_web::Body;
use std::path::Path;
use crate::infrastructure::SaveInfo;

#[test]
fn test_empty_payload_ok() {
    let resp = run_json_handler("{	\"file\": [], \"uri\": []}".to_owned());

    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[test]
fn test_json_base64_file_ok() {
    let json = fs::read_to_string("./test_data/payloads/one_file_json_payload").unwrap();
    let resp = run_json_handler(json);

    assert_eq!(resp.status(), http::StatusCode::OK);
    assert_body(resp.body(), 2);
}

#[test]
fn test_json_base64_multiple_files_ok() {
    let json = fs::read_to_string("./test_data/payloads/two_file_json_payload").unwrap();
    let resp = run_json_handler(json);

    assert_eq!(resp.status(), http::StatusCode::OK);
    assert_body(resp.body(), 4);
}

#[test]
fn test_json_uri_ok() {
    let json = fs::read_to_string("./test_data/payloads/one_file_json_payload").unwrap();
    let resp = run_json_handler(json);

    assert_eq!(resp.status(), http::StatusCode::OK);
    assert_body(resp.body(), 2);
}

#[test]
fn test_json_uri_and_file_ok() {
    let json = fs::read_to_string("./test_data/payloads/one_uri_two_file_json_payload").unwrap();
    let resp = run_json_handler(json);

    assert_eq!(resp.status(), http::StatusCode::OK);
    assert_body(resp.body(), 6);
}

fn run_json_handler(payload: String) -> HttpResponse {
    test::TestRequest::with_header("content-type", "application/json")
        .set_payload(payload)
        .run(&handle)
        .unwrap()
}

fn clear_uploads(files: &Vec<String>) {
    files.iter().for_each(|file| fs::remove_file(file).unwrap());
}

fn assert_body(body: &Body, expected_file_count: usize) {
    match *body {
        Body::Binary(ref b) => {
            let as_string = ::std::str::from_utf8(b.as_ref()).unwrap();
            let json = serde_json::from_str::<Vec<SaveInfo>>(as_string).unwrap();

            let files = json.into_iter()
                .flat_map(|info| {
                    if info.success_info.is_some() {
                        let success_info = info.success_info.unwrap();
                        let original_path = success_info.original_path.clone();
                        let resize_path = success_info.resize_path.clone();
                        vec![original_path, resize_path]
                    } else {
                        vec![]
                    }
                })
                .collect::<Vec<String>>();

            assert_eq!(expected_file_count, files.len());

            files.iter().for_each(|file| assert!(Path::new(file).exists()));

            clear_uploads(&files);
        }
        _ => panic!("assert fail, uncaught body type")
    }
}