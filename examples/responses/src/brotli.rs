// ANCHOR: brotli
use actix_web::{
    dev::BodyEncoding, get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};

#[get("/")]
async fn index_br() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Br)
        .body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index_br)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// ANCHOR_END: brotli
