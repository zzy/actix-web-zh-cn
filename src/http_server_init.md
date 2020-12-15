# HTTP 服务器初始化

> [http_server_init.md](https://github.com/actix/actix-website/blob/master/content/docs/http_server_init.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

## 架构总览

下面的序列图是 HttpServer 初始化过程，处理代码如下：

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::to(|| HttpResponse::Ok()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

![](./css/http_server.svg)
