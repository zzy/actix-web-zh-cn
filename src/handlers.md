# 请求处理

> [handlers.md](https://github.com/actix/actix-website/blob/master/content/docs/handlers.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

请求处理程序是异步函数，它接受零个或多个参数，这些参数可以从请求中提取（即实现了 `FromRequest` trait，参见 [*impl FromRequest*][implfromrequest]），并返回可以转换为 `HttpResponse` 的类型（即实现了  `Responder` trait，参见[*impl Responder*][respondertrait]）。

请求处理分为两个阶段。

- 首先，调用处理程序对象，返回实现了 [*Responder*][respondertrait] trait 的任何对象。
- 然后，对返回的对象调用 `respond_to()` 方法，将其自身转换为 `HttpResponse` 或者 `Error`。

默认情况下，actix-web 为一些标准类型提供了 `Responder` trait 实现。例如，`&'static str`、`String` 等。

> 已经实现 `Responder` trait 的类型，其完整清单请参见 [*Responder 文档*][responderimpls]。

请求处理程序示例： 

```rust,edition2018,no_run,noplaypen
async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}
```

```rust,edition2018,no_run,noplaypen
async fn index(_req: HttpRequest) -> String {
    "Hello world!".to_owned()
}
```

如果涉及到更复杂的类型，你还可以更改签名以返回比较好用的 `impl Responder`（实现 `Responder` trait）。

```rust,edition2018,no_run,noplaypen
async fn index(_req: HttpRequest) -> impl Responder {
    Bytes::from_static(b"Hello world!")
}
```

```rust,edition2018,no_run,noplaypen
async fn index(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    ...
}
```

## 响应自定义类型

要直接从处理程序函数返回自定义类型，则该类型需要实现 `Responder` trait。

让我们为一个自定义类型创建响应，该类型将序列化为 `application/json` 响应：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responder-trait/src/main.rs:responder-trait}}
```

## 流式响应体（body）

响应体可以异步生成。下述实例中，主体（body）必须实现 `stream` trait `Stream<Item=Bytes, Error=Error>`，即：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/async-handlers/src/stream.rs:stream}}
```

## 差异化返回类型（Either 枚举）

有时，你需要返回不同类型的响应。比如，你可以检查错误和返回错误：返回错误的异步响应，或者返回依赖于两个不同类型的任意结果（result）。

下述实例中，可以使用 [Either][either] 枚举类型，`Either` 允许将两种不同的响应类型组合成单一类型。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/either/src/main.rs:either}}
```

[implfromrequest]: https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html
[respondertrait]: https://docs.rs/actix-web/3/actix_web/trait.Responder.html
[responderimpls]: https://docs.rs/actix-web/3/actix_web/trait.Responder.html#foreign-impls
[either]: https://docs.rs/actix-web/3/actix_web/enum.Either.html
