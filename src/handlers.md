---
title: Handlers
menu: docs_basics
weight: 160
---

# Request Handlers

A request handler is an async function that accepts zero or more parameters that can be extracted
from a request (ie, [*impl FromRequest*][implfromrequest]) and returns a type that can
be converted into an HttpResponse (ie, [*impl Responder*][respondertrait]).

Request handling happens in two stages. First the handler object is called, returning any
object that implements the [*Responder*][respondertrait] trait. Then, `respond_to()` is
called on the returned object, converting itself to a `HttpResponse` or `Error`.

By default actix-web provides `Responder` implementations for some standard types,
such as `&'static str`, `String`, etc.

> For a complete list of implementations, check [*Responder documentation*][responderimpls].

Examples of valid handlers:

```rust
async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}
```

```rust
async fn index(_req: HttpRequest) -> String {
    "Hello world!".to_owned()
}
```

You can also change the signature to return `impl Responder` which works well if more
complex types are involved.

```rust
async fn index(_req: HttpRequest) -> impl Responder {
    Bytes::from_static(b"Hello world!")
}
```

```rust
async fn index(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    ...
}
```

## Response with custom type

To return a custom type directly from a handler function, the type needs to implement the `Responder` trait.

Let's create a response for a custom type that serializes to an `application/json` response:

{{< include-example example="responder-trait" file="main.rs" section="responder-trait" >}}

## Streaming response body

Response body can be generated asynchronously. In this case, body must implement
the stream trait `Stream<Item=Bytes, Error=Error>`, i.e:

{{< include-example example="async-handlers" file="stream.rs" section="stream" >}}

## Different return types (Either)

Sometimes, you need to return different types of responses. For example, you can error
check and return errors, return async responses, or any result that requires two different types.

For this case, the [Either][either] type can be used.  `Either` allows combining two
different responder types into a single type.

{{< include-example example="either" file="main.rs" section="either" >}}

[implfromrequest]: https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html
[respondertrait]: https://docs.rs/actix-web/3/actix_web/trait.Responder.html
[responderimpls]: https://docs.rs/actix-web/3/actix_web/trait.Responder.html#foreign-impls
[either]: https://docs.rs/actix-web/3/actix_web/enum.Either.html
