// ANCHOR: signals
use actix_web::{web, App, HttpResponse, HttpServer, rt::System};
use std::sync::mpsc;
use std::thread;

#[actix_web::main]
async fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let sys = System::new("http-server");

        let srv = HttpServer::new(|| {
            App::new().route("/", web::get().to(|| HttpResponse::Ok()))
        })
        .bind("127.0.0.1:8080")?
        .shutdown_timeout(60) // <- Set shutdown timeout to 60 seconds
        .run();

        let _ = tx.send(srv);
        sys.run()
    });

    let srv = rx.recv().unwrap();

    // 暂停接受传入的连接
    srv.pause().await;
    // 重新开始接受传入的连接
    srv.resume().await;
    // 停止服务器
    srv.stop(true).await;
}
// ANCHOR_END: signals
