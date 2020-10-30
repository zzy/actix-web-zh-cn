#![allow(dead_code, unused_variables)]

// ANCHOR: wrap-fn
use actix_service::Service;
use actix_web::{web, App};
use futures::future::FutureExt;

#[actix_web::main]
async fn main() {
    let app = App::new()
        .wrap_fn(|req, srv| {
            println!("Hi from start. You requested: {}", req.path());
            srv.call(req).map(|res| {
                println!("Hi from response");
                res
            })
        })
        .route(
            "/index.html",
            web::get().to(|| async {
                "Hello, middleware!"
            }),
        );
}
// ANCHOR_END: wrap-fn
