# 错误

> [errors.md](https://github.com/actix/actix-website/blob/master/content/docs/errors.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

actix-web 使用它自带的 [`actix_web::error::Error`][actixerror] 类型和 [`actix_web::error::ResponseError`][responseerror] trait 来处理 web `handler` 的错误。

如果 `handler` 在实现了 `ResponseError` trait 的 `Result` 中返回 `Error`（具体指[常规的 Rust trait
`std::error::Error`][stderror]），则 actix-web 会将该错误呈现为一个 HTTP 响应，并使用相应的状态码 [`actix_web::http::StatusCode`][status_code]。默认情况下，内部服务器会产生错误：

```rust
pub trait ResponseError {
    fn error_response(&self) -> Response<Body>;
    fn status_code(&self) -> StatusCode;
}
```

`Responder` trait 会将兼容的 `Result` 强制转换为 HTTP 响应：

```rust
impl<T: Responder, E: Into<Error>> Responder for Result<T, E>
```

上述源代码中的 `Error` 是 actix-web 的错误定义，任何实现了 `ResponseError` trait 的错误都可以被自动转换。

actix-web 为一些常见的 non-actix（非 actix 相关）错误提供了 `ResponseError` trait 实现。例如，如果 `handler` 以 `io::Error` 响应，则该错误将转换为 `HttpInternalServerError`：

```rust
use std::io;
use actix_files::NamedFile;

fn index(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}
```

已经实现 `ResponseError` trait 的外部类型，其完整清单请参见 [actix-web API 文档][responseerrorimpls]。

## 自定义错误响应

下属代码是实现了 `ResponseError` trait 的示例，它使用 [derive_more] crate 来声明错误枚举。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/main.rs:response-error}}
```

`ResponseError` 有一个默认的 `error_response()` 实现，它将渲染一个 _HTTP-500_ 错误（内部服务器错误）。上文示例代码中，当我们执行 `index` handler 时，就会发生这种 _HTTP-500_ 错误。

`error_response()` 方法可以重写，以生成更有用的结果：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/override_error.rs:override}}
```

# 错误助手

actix-web 提供了一系列错误助手（错误帮助程序）函数。在从其它错误生成特定的 HTTP 错误代码时，这些函数对于非常有用。下文的示例中，结构体 `MyError` 并未实现 `ResponseError` trait，我们使用 `map_err` 函数将其转换为 _HTTP-400_ 错误（错误请求）：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/helpers.rs:helpers}}
```

有关可用的错误帮助程序的完整列表，请参阅[actix-web 中 `error` 模块的 API 文档][actixerror]。

# 错误日志

在 `WARN` 日志级别，actix 记录所有错误。如果应用程序的日志级别设置为 `DEBUG`，并且启用了回溯功能 `RUST_BACKTRACE`，则回溯日志也会被记录。这些可通过环境变量进行配置：

```
>> RUST_BACKTRACE=1 RUST_LOG=actix_web=debug cargo run
```

可用情况下，`Error` 类型会使用具体请求的错误回溯。如果是底层失败而没有提供回溯，则会构造一个新的回溯，指向发生错误转换的位置（而不是错误的起源）。

# 错误处理的推荐方式

考虑将应用程序产生的错误分为两大类：面向用户的错误、不面向用户的错误。

面向用户的错误的一个例子：我们可以为可能发生的失败情形，指定一个 `UserError` 枚举，该枚举封装了 `ValidationError`，以便于在用户发送错误的输入时，返回验证信息：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/recommend_one.rs:recommend-one}}
```

这将完全按照预期的方式运行，因为使用 `display` 定义的错误消息，其编写的明确意图即是为了用户读取。

然而，并不是所有错误都需要发回错误消息——在服务器环境中会发生许多错误，我们可能希望对用户隐藏细节。例如，如果数据库关闭，客户端会产生连接超时错误；或者 HTML 模板格式不正确，并且在呈现时出错。在这些情况下，最好将错误映射为适合用户处理的通用错误。

下面的示例代码中，使用自定义消息将内部错误映射为面向用户的 `InternalError`：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/recommend_two.rs:recommend-two}}
```

通过将错误分为面向用户的错误和不面向用户的错误，我们可以确保不会意外地将应用程序内部错误暴露给用户——这些错误是用户不希望看到的。

# 记录错误

使用日志中间件 `middleware::Logger` 记录错误日志的示例：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/logging.rs:logging}}
```

[actixerror]: https://docs.rs/actix-web/3/actix_web/error/struct.Error.html
[errorhelpers]: https://docs.rs/actix-web/3/actix_web/trait.ResponseError.html
[derive_more]: https://crates.io/crates/derive_more
[responseerror]: https://docs.rs/actix-web/3/actix_web/error/trait.ResponseError.html
[responseerrorimpls]:
  https://docs.rs/actix-web/3/actix_web/error/trait.ResponseError.html#foreign-impls
[stderror]: https://doc.rust-lang.org/std/error/trait.Error.html
[status_code]: https://docs.rs/actix-web/3.0.0/actix_web/http/struct.StatusCode.html
