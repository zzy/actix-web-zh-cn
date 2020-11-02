# actix-web 中文文档

actix-web 是 Rust 生态中的最为优秀的 web 框架之一，具有类型安全、功能丰富、扩展性强，以及速度极快的诸多优点。

## 总览

让我们通过 actix-web 的典型代码，来对其做一个整体认知。

```rust,edition2018,no_run
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### 类型安全

忘掉字符串类型的对象吧，从请求到响应，所有的数据信息都有类型。

### 功能丰富

actix 提供了诸多开箱即用的功能和特性，如 HTTP/2、日志记录等。

### 扩展性强

轻松创建自定义库，任何 actix 应用程序都可以无缝集成。

### 速度极快

Actix 具有极快的速度，请参见 [techempower 性能基准测试](https://www.techempower.com/benchmarks/#section=data-r19)。

## 实践

### 灵活响应

actix 中的处理函数可以返回大量实现了 `Responder` trait 的对象，这使得从诸多 API 返回一致的响应变得轻而易举。

```rust,edition2018,no_run
#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

async fn hello_world() -> impl Responder {
    "Hello World!"
}

async fn current_temperature() -> impl Responder {
    web::Json(Measurement { temperature: 42.3 })
}
```

## 增强萃取

actix 自实现了一个强大的提取器系统，可以从传入的 HTTP 请求中提取数据，并将其传递给视图函数。这不仅有助于实现一个简捷的 API，而且还意味着你的视图函数可以是同步代码，但仍然受益于异步 IO 处理。

```rust,edition2018,no_run
#[derive(Deserialize, Serialize)]
struct Event {
    id: Option<i32>,
    timestamp: f64,
    kind: String,
    tags: Vec<String>,
}

async fn capture_event(evt: web::Json<Event>) -> impl Responder {
    let new_event = store_in_db(evt.timestamp, &evt.kind, &evt.tags);
    format!("got event {}", new_event.id.unwrap())
}
```

### 简便的表单处理

处理 multipart/urlencoded 表单数据很容易。只需定义一个可以反序列化的结构，actix 将处理其余部分。

```rust,edition2018,no_run
#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

async fn register(form: web::Form<Register>) -> impl Responder {
    format!("Hello {} from {}!", form.username, form.country)
}
```

### 具备请求路由

actix 具备 URL 路由系统，可以匹配 URL 并调用各个 `handler`。为了获得额外的灵活性，可以使用作用域。

```rust,edition2018,no_run
#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page!"
}

async fn hello(path: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &path)
}

let app = App::new()
    .service(index)
    .route("/{name}", web::get().to(hello));
```
