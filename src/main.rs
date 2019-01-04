extern crate actix_web;

use image_pool::web;

use actix_web::{
    server, App, HttpResponse,
};

use actix_web::http::Method;
use actix_web::pred;

fn main() {
    server::new(||
        App::new()
            .resource("/images", |r| {
                r.method(Method::PUT).filter(pred::Header("content-type", "application/json")).a(web::json_handler::handle);
                r.method(Method::PUT).filter(pred::Not(pred::Header("content-type", "application/json"))).a(web::multipart_handler::handle);

                r.route().filter(pred::Not(pred::Put())).f(|req| {
                    println!("{:?}", req.headers());
                    HttpResponse::MethodNotAllowed()
                });
            })
    )
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}