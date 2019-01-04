#![cfg(test)]

use std::fs;
use crate::infrastructure::service;
use bytes::Bytes;
use crate::infrastructure::SaveInfo;
use std::path::Path;
use actix::System;

#[test]
fn test_raw_file_saving() {
    const TEST_DIR: &str = "./temp_test_dir/service-temp-test1";
    setup(TEST_DIR);

    let data = fs::read("./test_data/2d.png").expect("Cant read img file");
    let data = Bytes::from(data);
    let result = service::save_from_file(Ok((data, mime::IMAGE_PNG, "img.png".to_owned())), TEST_DIR).unwrap();

    assert!(result.ok);
    assert_result_files(result);

    teardown(TEST_DIR);
}

#[test]
fn test_raw_file_fail_info_when_bad_mime() {
    const TEST_DIR: &str = "./temp_test_dir/service-temp-test2";
    setup(TEST_DIR);

    let data = fs::read("./test_data/2d.png").expect("Cant read img file");
    let data = Bytes::from(data);
    let result = service::save_from_file(Ok((data, mime::IMAGE_STAR, "img.png".to_owned())), TEST_DIR).unwrap();

    assert!(!result.ok);
    assert!(result.fail_info.is_some());

    teardown(TEST_DIR);
}

#[test]
// libpng пишет ворнинги на этом тесте, тк opencv дает сохранить изначальный вектор (но кидает ошибку при ресайзе)
fn test_raw_file_fail_info_when_broken_data() {
    const TEST_DIR: &str = "./temp_test_dir/service-temp-test3";
    setup(TEST_DIR);

    let data = Bytes::from(vec![1,2,3,4,5]);
    let result = service::save_from_file(Ok((data, mime::IMAGE_PNG, "img.png".to_owned())), TEST_DIR).unwrap();

    assert!(!result.ok);
    assert!(result.fail_info.is_some());

    teardown(TEST_DIR);
}

#[test]
fn test_uri_saving() {
    const TEST_DIR: &str = "./temp_test_dir/service-temp-test4";
    setup(TEST_DIR);

    let result_fut = service::save_from_uri("https://www.rust-lang.org/static/images/rust-logo-blk.png".to_owned(), TEST_DIR);

    let mut ctx = System::new("test");
    let save_response = ctx.block_on(result_fut).unwrap();

    assert!(save_response.ok);
    assert_result_files(save_response);

    teardown(TEST_DIR);
}

#[test]
fn test_uri_fail_info_when_broken_uri() {
    const TEST_DIR: &str = "./temp_test_dir/service-temp-test5";
    setup(TEST_DIR);

    let result_fut = service::save_from_uri("https://qq/rust-logo-blk.png".to_owned(), TEST_DIR);

    let mut ctx = System::new("test");
    let result = ctx.block_on(result_fut);

    assert!(result.is_err());

    teardown(TEST_DIR);
}

fn assert_result_files(result: SaveInfo) {
    let success_info = result.success_info.unwrap();

    let original_fp =  &success_info.original_path;
    let resize_fp =  &success_info.resize_path;

    assert!(Path::new(original_fp).exists());
    assert!(Path::new(resize_fp).exists());
}

fn setup(dir: &str) {
    fs::create_dir_all(dir).unwrap();
}

fn teardown(dir: &str) {
    fs::remove_dir_all(dir).unwrap();
}
