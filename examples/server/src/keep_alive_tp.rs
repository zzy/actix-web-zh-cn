// <example>
use actix_web::{http, HttpRequest, HttpResponse};

async fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .connection_type(http::ConnectionType::Close) // <- Close connection
        .force_close() // <- Alternative method
        .finish()
}
// </example>

// ConnectionType::Close
// ConnectionType::KeepAlive
// ConnectionType::Upgrade
