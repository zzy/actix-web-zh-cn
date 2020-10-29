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

[*路径（Path）*][pathstruct]结构体提供可从请求路径提取的信息，路径中的任何变量都可以反序列化。

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

[*查询（Query）*][querystruct]结构体为请求中的查询参数提供提取功能。下文的例子使用了 *serde_urlencoded* crate：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/extractors/src/query.rs:query}}
```

# Json

[*Json*][jsonstruct] 结构体允许将请求体反序列化为结构体。要从请求体中提取类型化的信息，则类型 `T` 必须实现 *serde* crate 中的 `Deserialize` trait。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/extractors/src/json_one.rs:json-one}}
```

一些提取器提供了配置提取过程的方法，[*JsonConfig*][jsonconfig] 结构体用于配置 Json 提取器。要配置提取器，请将其配置对象传递给 `web::resource` 的 `.app_data()` 方法。配置后，*Json* 提取器将返回 *JsonConfig* 结构体。你也可以配置 json 有效负载的最大值，以及自定义错误处理函数。

下面的示例中，将有效负载的大小限制为 4kb，并使用自定义的错误处理程序。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/extractors/src/json_two.rs:json-two}}
```

# 表单（Form）

目前，仅支持 url 编码的表单。url 编码的主体信息可以被提取为特定类型，此类型必须实现 *serde* crate 中的 `Deserialize` trait。

[*FormConfig*][formconfig] 结构体允许配置提取过程。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/extractors/src/form.rs:form}}
```

# 其它

actix-web 还提供了其它几种提取器：

* [*Data*][datastruct] - 如果需要访问应用程序状态。
* *HttpRequest* - *HttpRequest* 自身既是提取器，它返回 `self`，以便于你访问请求。
* *String* - 你可以转换请求的有效负载为 *字符串（String）* 类型。请参阅文档字符串[*实例*][stringexample]。
* *bytes::Bytes* - 你可以转换请求的有效负载为 *Bytes* 类型。请参阅文档字符串[*实例*][bytesexample]。
* *Payload* - 你可以访问请求的有效负载。请参阅[*实例*][payloadexample]。

# 应用状态提取器

可以使用 `web::Data` 提取器，从请求处理程序访问应用程序状态；但是，状态仅可以作为只读引用访问。如果你需要对状态的可变（mutable）访问，则状态必须被实现。

注意，actix

> **注意**，actix 会创建应用程序状态和请求处理程序的多个副本，它为每个工作线程创建一个副本。

下面是一个请求处理程序的示例，用于存储已处理的请求数：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/request-handlers/src/main.rs:data}}
```

尽管此处理程序可以运行，但依赖于线程数和每个线程处理的请求数因素，`self.0` 可能不正确。正确的实现应该使用 `Arc（原子引用计数器）` 和 `AtomicUsize`。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/request-handlers/src/handlers_arc.rs:arc}}
```

> `actix-web` 框架异步地处理请求，请小心使用诸如 `Mutex` 或者 `RwLock` 之类的同步原语。
> 如果阻止了线程执行，所有并发请求处理进程都将阻塞。
> 若你需要从多个线程共享或更新某些状态，请考虑使用 `tokio` crate 的同步原语。

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
