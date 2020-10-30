---
title: Responses
menu: docs_advanced
weight: 210
---

# Response

A builder-like pattern is used to construct an instance of `HttpResponse`.  `HttpResponse`
provides several methods that return a `HttpResponseBuilder` instance, which implements
various convenience methods for building responses.

> Check the [documentation][responsebuilder] for type descriptions.

The methods `.body`, `.finish`, and `.json` finalize response creation and return a
constructed *HttpResponse* instance. If this methods is called on the same builder
instance multiple times, the builder will panic.

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/main.rs:builder}}
```

# Content encoding

Actix-web can automatically *compress* payloads with the [*Compress middleware*][compressmidddleware].
The following codecs are supported:

* Brotli
* Gzip
* Deflate
* Identity

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/compress.rs:compress}}
```

Response payload is compressed based on the *encoding* parameter from the
`middleware::BodyEncoding` trait.  By default, `ContentEncoding::Auto` is used. If
`ContentEncoding::Auto` is selected, then the compression depends on the request's
`Accept-Encoding` header.

> `ContentEncoding::Identity` can be used to disable compression.
> If another content encoding is selected, the compression is enforced for that codec.

For example, to enable `brotli` for a single handler use `ContentEncoding::Br`:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/brotli.rs:brotli}}
```

or for the entire application:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/brotli_two.rs:brotli-two}}
```

In this case we explicitly disable content compression by setting content encoding to
an `Identity` value:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/identity.rs:identity}}
```

When dealing with an already compressed body (for example when serving assets),
set the content encoding to `Identity` to avoid compressing the already compressed
data and set the `content-encoding` header manually:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/identity_two.rs:identity-two}}
```

Also it is possible to set default content encoding on application level, by
default `ContentEncoding::Auto` is used, which implies automatic content compression
negotiation.

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/auto.rs:auto}}
```

# JSON Response

The `Json` type allows to respond with well-formed JSON data: simply return a value of
type `Json<T>` where `T` is the type of a structure to serialize into *JSON*.
The type `T` must implement the `Serialize` trait from *serde*.

```rust,edition2018,no_run,noplaypen
{{#include ../examples/responses/src/json_resp.rs:json-resp}}
```

[responsebuilder]: https://docs.rs/actix-web/3/actix_web/dev/struct.HttpResponseBuilder.html
[compressmidddleware]: https://docs.rs/actix-web/3/actix_web/middleware/struct.Compress.html
