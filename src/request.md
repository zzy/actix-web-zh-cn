# 请求

> [request.md](https://github.com/actix/actix-website/blob/master/content/docs/request.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

## 内容密码

actix-web 可对有效负载自动*解压缩*。支持如下编解码器：

* Brotli
* Chunked
* Compress
* Gzip
* Deflate
* Identity
* Trailers
* EncodingExt

如果请求消息标头包含 `Content-Encoding` 消息标头，则根据消息标头值解压缩请求负载。不支持多个编解码器，即：`Content-Encoding: br, gzip`。

## JSON 请求

对于 json 主体正文的反序列化，有多个选项。

第一个选项是使用 *Json* 提取器。首先，定义一个 handler 函数，接受 `Json<T>` 作为参数；然后，使用 `.to()` 方法注册该 handler。也可以通过使用 `serde_json::Value`，作为类型 `T` 来接受任意有效的 json 对象。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/requests/src/main.rs:json-request}}
```

您还可以手动将有效负载加载到内存中，然后对其反序列化。

在下面的示例中，我们将反序列化 *MyObj* 结构体。我们需要先加载请求主体，然后将 json 反序列化到一个对象中。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/requests/src/manual.rs:json-manual}}
```

> [实例文件夹][examples]中提供了这两个选项的完整示例。

## 分块编码

actix 对*分块* 编码自动解码。[`web::Payload`][payloadextractor] 提取器已包含解码的字节流。如果请求负载是用支持的压缩编解码器（br、gzip、deflate）压缩的，则字节流将被解压缩。

## Multipart 主体

actix-web 通过扩展的 crate [`actix-multipart`][multipartcrate] 提供了 multipart 流支持。

> [示例文件夹][multipartexample]中提供了完整示例。

## Urlencoded 主体

actix-web 提供了对 *application/x-www-form-urlencoded* 编码主体的支持，其中 [`web::Form`][formencoded] 提取器可解析为反序列化的实例。实例的类型必须实现 *serde* crate 提供的 `Deserialize` trait。


在以下几种情况中，*UrlEncoded* future 会解析为错误：

* content-type 不是 `application/x-www-form-urlencoded`。
* 传输`分块（chunked）`编码。
* content-length 大于 256k。
* 有效负载因错误而终止。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/requests/src/urlencoded.rs:urlencoded}}
```

## 流式请求

*HttpRequest* 是一个`字节（Bytes）`流对象，可以用来读取请求主体的有效负载。

下面的示例中，我们逐块读取并打印请求负载：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/requests/src/streaming.rs:streaming}}
```

[examples]: https://github.com/actix/examples/tree/master/json/
[multipartstruct]: https://docs.rs/actix-multipart/0.2/actix_multipart/struct.Multipart.html
[fieldstruct]: https://docs.rs/actix-multipart/0.2/actix_multipart/struct.Field.html
[multipartexample]: https://github.com/actix/examples/tree/master/multipart/
[urlencoded]: https://docs.rs/actix-web/3/actix_web/dev/struct.UrlEncoded.html
[payloadextractor]: https://docs.rs/actix-web/3/actix_web/web/struct.Payload.html
[multipartcrate]: https://crates.io/crates/actix-multipart
[formencoded]:Jhttps://docs.rs/actix-web/3/actix_web/web/struct.Form.html
