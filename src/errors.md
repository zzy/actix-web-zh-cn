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

## An example of a custom error response

Here's an example implementation for `ResponseError`, using the [derive_more] crate
for declarative error enums.

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/main.rs:response-error}}
```

`ResponseError` has a default implementation for `error_response()` that will render a _500_
(internal server error), and that's what will happen when the `index` handler executes above.

Override `error_response()` to produce more useful results:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/override_error.rs:override}}
```

# Error helpers

Actix-web provides a set of error helper functions that are useful for generating specific HTTP
error codes from other errors. Here we convert `MyError`, which doesn't implement the
`ResponseError` trait, to a _400_ (bad request) using `map_err`:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/helpers.rs:helpers}}
```

See the [API documentation for actix-web's `error` module][actixerror] for a full list of available
error helpers.

# Error logging

Actix logs all errors at the `WARN` log level. If an application's log level is set to `DEBUG` and
`RUST_BACKTRACE` is enabled, the backtrace is also logged. These are configurable with environmental
variables:

```
>> RUST_BACKTRACE=1 RUST_LOG=actix_web=debug cargo run
```

The `Error` type uses the cause's error backtrace if available. If the underlying failure does not
provide a backtrace, a new backtrace is constructed pointing to the point where the conversion
occurred (rather than the origin of the error).

# Recommended practices in error handling

It might be useful to think about dividing the errors an application produces into two broad groups:
those which are intended to be be user-facing, and those which are not.

An example of the former is that I might use failure to specify a `UserError` enum which
encapsulates a `ValidationError` to return whenever a user sends bad input:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/recommend_one.rs:recommend-one}}
```

This will behave exactly as intended because the error message defined with `display` is written
with the explicit intent to be read by a user.

However, sending back an error's message isn't desirable for all errors -- there are many failures
that occur in a server environment where we'd probably want the specifics to be hidden from the
user. For example, if a database goes down and client libraries start producing connect timeout
errors, or if an HTML template was improperly formatted and errors when rendered. In these cases, it
might be preferable to map the errors to a generic error suitable for user consumption.

Here's an example that maps an internal error to a user-facing `InternalError` with a custom
message:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/errors/src/recommend_two.rs:recommend-two}}
```

By dividing errors into those which are user facing and those which are not, we can ensure that we
don't accidentally expose users to errors thrown by application internals which they weren't meant
to see.

# Error Logging

This is a basic example using `middleware::Logger`:

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
