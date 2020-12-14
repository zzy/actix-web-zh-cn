# 响应

> [response.md](https://github.com/actix/actix-website/blob/master/content/docs/response.md)
> <br />
> commit - 47473dc4bbd1a54ef135bea5cac1d04590dba986 - 2020.09.19

使用类似于构建器的模式构造 `HttpResponse` 实例。`HttpResponse` 提供了返回 `HttpResponseBuilder` 实例的几种方法，`HttpResponseBuilder` 实例实现了多种便捷方法用于生成响应。

> 可查看[文档][responsebuilder]中的类型描述。

方法 `.body`、`.finish`，以及 `.json` 完成响应创建，并返回一个已构造的 *HttpResponse* 实例。如果在同一构造器实例上多次调用此方法，构造器将会抛出异常。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/main.rs:builder}}
```

## 内容编码

actix-web 可以使用 [*Compress 中间价*][compressmidddleware]自动*压缩* 有效负载。支持以下编解码器：

* Brotli
* Gzip
* Deflate
* Identity

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/compress.rs:compress}}
```

响应负载基于中间价 `middleware::BodyEncoding` trait 中的 *encoding* 参数进行压缩。默认情况下，使用参数 `ContentEncoding::Auto`。如果选择使用参数 `ContentEncoding::Auto`，则压缩取决于请求的 `Accept-Encoding` 消息标头。

> `ContentEncoding::Identity` 可用于禁用压缩。如果选择了其它内容编码方式，则对该编解码器强制压缩。

例如，要为单个 handler 启用 `brotli`，请使用 `ContentEncoding::Br`：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/brotli.rs:brotli}}
```

或者对于整个应用程序：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/brotli_two.rs:brotli-two}}
```

在这种情况下，我们通过将内容编码设置为 `Identity` 值来显式禁用内容压缩：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/identity.rs:identity}}
```

在处理已压缩的正文主体（例如处理服务器静态资源文件）时，将内容编码设置为 `Identity`，以避免压缩已压缩的数据，同时需要手动设置 `content-encoding` 消息表头：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/identity_two.rs:identity-two}}
```

也可以在应用程序级别设置默认内容编码，默认情况下使用 `ContentEncoding::Auto`，这意味着可修改的自动内容压缩。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/auto.rs:auto}}
```

## JSON 响应

`Json` 类型允许使用符合语法规则的 Json 数据进行响应：只需返回 `Json<T>` 类型的值，其中 `T` 是要序列化为 *JSON* 的结构类型。类型 `T` 必须实现 *serde* crate 中的 `Serialize` trait。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/json_resp.rs:json-resp}}
```

[responsebuilder]: https://docs.rs/actix-web/3/actix_web/dev/struct.HttpResponseBuilder.html
[compressmidddleware]: https://docs.rs/actix-web/3/actix_web/middleware/struct.Compress.html
