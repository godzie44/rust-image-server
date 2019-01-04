#![cfg(test)]

use actix_web::{http, test, HttpResponse};
use actix_web::App;
use actix_web::http::Method;
use actix_web::pred;
use crate::web;

fn create_app() -> App {
    App::new()
        .resource("/images", |r| {
            r.method(Method::PUT).filter(pred::Not(pred::Header("content-type", "application/json"))).f(web::multipart_handler::handle);

            r.route().filter(pred::Not(pred::Put())).f(|req| {
                println!("{:?}", req.headers());
                HttpResponse::MethodNotAllowed()
            });
        })
}

#[test]
fn test_empty_form_bad_request() {
    let mut srv = test::TestServer::with_factory(create_app);

    let request = srv.client(
        http::Method::PUT, "/images")
        .header("content-type", "multipart/form-data; boundary=--------KD")
        .finish()
        .unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status().as_str(), "400");
}