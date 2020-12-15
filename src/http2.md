# HTTP/2

> [http2.md](https://github.com/actix/actix-website/blob/master/content/docs/http2.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

`actix-web` automatically upgrades connections to *HTTP/2* if possible.

## Negotiation

*HTTP/2* protocol over TLS without prior knowledge requires [TLS ALPN][tlsalpn].

<!-- TODO: use rustls example -->
> Currently, only `rust-openssl` has support.

`alpn` negotiation requires enabling the feature. When enabled, `HttpServer` provides the
[bind_openssl][bindopenssl] method.

```toml
[dependencies]
actix-web = { version = "{{< actix-version "actix-web" >}}", features = ["openssl"] }
openssl = { version = "0.10", features = ["v110"] }
```

```rust,edition2018,no_run,noplaypen
{{#include ../examples/http2/src/main.rs:main}}
```

Upgrades to *HTTP/2.0* schema described in [rfc section 3.2][rfcsection32] is not
supported.  Starting *HTTP/2* with prior knowledge is supported for both clear text
connection and tls connection. [rfc section 3.4][rfcsection34].

> Check out [examples/tls][examples] for a concrete example.

[rfcsection32]: https://http2.github.io/http2-spec/#rfc.section.3.2
[rfcsection34]: https://http2.github.io/http2-spec/#rfc.section.3.4
[bindopenssl]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind_openssl
[tlsalpn]: https://tools.ietf.org/html/rfc7301
[examples]: https://github.com/actix/examples/tree/master/rustls
