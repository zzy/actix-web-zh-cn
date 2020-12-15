# HTTP/2

> [http2.md](https://github.com/actix/actix-website/blob/master/content/docs/http2.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

`actix-web` 可自动升级连接协议为 *HTTP/2*。

## 协商

基于 TLS 的 *HTTP/2* 协议需要 [TLS ALPN][tlsalpn]。

<!-- TODO: use rustls example -->
> 当前，仅有 `rust-openssl` 提供了支持。

`alpn` 协商需要启用该功能。启用后，`HttpServer` 提供了 [bind_openssl][bindopenssl] 方法。

```toml
[dependencies]
actix-web = { version = "{{< actix-version "actix-web" >}}", features = ["openssl"] }
openssl = { version = "0.10", features = ["v110"] }
```

```rust,edition2018,no_run,noplaypen
{{#include ../examples/http2/src/main.rs:main}}
```

不支持升级到 [rfc 章节 3.2][rfcsection32] 中描述的 *HTTP/2.0* 架构模式，但明文连接和 tls 连接都支持，需要在事先确定的情况下启动 *HTTP/2*。详见[rfc 章节 3.4][rfcsection34]。

> 查看 [examples/tls][examples] 以获取具体示例。

[rfcsection32]: https://http2.github.io/http2-spec/#rfc.section.3.2
[rfcsection34]: https://http2.github.io/http2-spec/#rfc.section.3.4
[bindopenssl]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind_openssl
[tlsalpn]: https://tools.ietf.org/html/rfc7301
[examples]: https://github.com/actix/examples/tree/master/rustls
