# 类型安全的信息提取

> [extractors.md](https://github.com/actix/actix-website/blob/master/content/docs/extractors.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

actix-web 提供了一个称之为*提取器*（extractor，`impl FromRequest`）的请求信息访问工具，它是类型安全的。默认情况下，actix-web 提供了多种提取器实现。

提取器可以作为处理程序函数的参数。actix-web 支持每个处理程序函数最多有 10 个提取器参数，参数位置无关紧要。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/extractors/src/main.rs:option-one}}
```

# 路径（Path）

[*路径（Path）*][pathstruct]是结构体类型，提供可从请求路径提取的信息，路径中的任何变量都可以反序列化。

举例来说，对于注册为 `/users/{user_id}/{friend}` 路径的资源，有两个变量可以被反序列化：`user_id` 和 `friend`。这些变量可以被提取到一个`元组（tuple）`中（如 `Path<(u32, String)>`），或者被提取到实现了 *serde* crate 中的 `Deserialize` trait 的任何结构中。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/extractors/src/path_one.rs:path-one}}
```

路径信息也可以提取到实现了 *serde* crate 中的 `Deserialize` trait 的特定类型中。下面是一个使用*结构体（struct）*类型而非*元组（tuple）*类型的例子，结构体类型实现了 *serde* crate 中的 `Deserialize` trait，它和使用*元组（tuple）*类型是等效的。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/extractors/src/path_two.rs:path-two}}
```

还可以使用 `get` 方法或者 `query` 方法，根据参数名称提取请求中的路径参数：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/extractors/src/path_three.rs:path-three}}
```

# 查询（Query）

[*查询（Query）*][querystruct]是结构体类型，为请求中的查询参数提供提取功能。下文的例子使用了 *serde_urlencoded* crate：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/extractors/src/query.rs:query}}
```

# Json

[*Json*][jsonstruct] allows deserialization of a request body into a struct. To extract
typed information from a request's body, the type `T` must implement the `Deserialize`
trait from *serde*.

{{< include-example example="extractors" file="json_one.rs" section="json-one" >}}

Some extractors provide a way to configure the extraction process. Json extractor
[*JsonConfig*][jsonconfig] type for configuration. To configure an extractor, pass its
configuration object to the resource's `.data()` method. In case of a *Json* extractor
it returns a *JsonConfig*. You can configure the maximum size of the json payload as
well as a custom error handler function.

The following example limits the size of the payload to 4kb and uses a custom error handler.

{{< include-example example="extractors" file="json_two.rs" section="json-two" >}}

# Form

At the moment, only url-encoded forms are supported. The url-encoded body could be
extracted to a specific type. This type must implement the `Deserialize` trait from
the *serde* crate.

[*FormConfig*][formconfig] allows configuring the extraction process.

{{< include-example example="extractors" file="form.rs" section="form" >}}

# Other

Actix-web also provides several other extractors:

* [*Data*][datastruct] - If you need access to an application state.
* *HttpRequest* - *HttpRequest* itself is an extractor which returns self, in case you
  need access to the request.
* *String* - You can convert a request's payload to a *String*.  [*Example*][stringexample]
  is available in doc strings.
* *bytes::Bytes* - You can convert a request's payload into *Bytes*.
  [*Example*][bytesexample]
  is available in doc strings.
* *Payload* - You can access a request's payload.
  [*Example*][payloadexample]

# Application state extractor

Application state is accessible from the handler with the `web::Data` extractor;
however, state is accessible as a read-only reference. If you need mutable access to state,
it must be implemented.

> **Beware**, actix creates multiple copies of the application state and the handlers. It creates
> one copy for each thread.

Here is an example of a handler that stores the number of processed requests:

{{< include-example example="request-handlers" file="main.rs" section="data" >}}

Although this handler will work, `self.0` will be different depending on the number of threads and
number of requests processed per thread. A proper implementation would use `Arc` and `AtomicUsize`.

{{< include-example example="request-handlers" file="handlers_arc.rs" section="arc" >}}

> Be careful with synchronization primitives like `Mutex` or `RwLock`. The `actix-web` framework
> handles requests asynchronously. By blocking thread execution, all concurrent
> request handling processes would block. If you need to share or update some state
> from multiple threads, consider using the tokio synchronization primitives.

[pathstruct]: https://docs.rs/actix-web/3/actix_web/dev/struct.Path.html
[querystruct]: https://docs.rs/actix-web/3/actix_web/web/struct.Query.html
[jsonstruct]: https://docs.rs/actix-web/3/actix_web/web/struct.Json.html
[jsonconfig]: https://docs.rs/actix-web/3/actix_web/web/struct.JsonConfig.html
[formconfig]: https://docs.rs/actix-web/3/actix_web/web/struct.FormConfig.html
[datastruct]: https://docs.rs/actix-web/3/actix_web/web/struct.Data.html
[stringexample]: https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html#example-2
[bytesexample]: https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html#example-4
[payloadexample]: https://docs.rs/actix-web/3/actix_web/web/struct.Payload.html
[actix]: https://actix.github.io/actix/actix/
