# 数据库

> [databases.md](https://github.com/actix/actix-website/blob/master/content/docs/databases.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

## 异步选项

我们有几个示例项目展示了异步数据库适配器的使用：

- SQLx: https://github.com/actix/examples/tree/master/sqlx_todo
- Postgres: https://github.com/actix/examples/tree/master/async_pg
- SQLite: https://github.com/actix/examples/tree/master/async_db

## Diesel

当前版本的 Diesel（v1）不支持异步操作，因此使用 [`web::block`][web-block] 函数是非常重要的，此函数可以将数据库操作加载到 actix 的运行时线程池。

您可以创建操作函数，对应到应用程序对数据库执行的所有操作。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/databases/src/main.rs:handler}}
```

您应该使用 `r2d2` 之类的 crate 来设置数据库池，这使得多个数据库连接可用于您的应用程序。还意味着多个 handler 可以同时操作数据库，并且仍然能够接受新的连接。简单地说，数据库连接池也是应用程序的状态（此种情况下，最好不要使用包裹状态为结构体，因为连接池为您处理共享访问）。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/databases/src/main.rs:main}}
```

在请求 handler 中，使用 `Data<T>` 提取器从应用程序状态（app state）中获取数据库连接池，然后从中获取连接。这提供了一个可以传递到 [`web::block`][web-block] 闭包中的数据库连接，并对其具有所有权（owned）。然后，使用必要的参数调用操作函数，最后通过 `.await` 返回结果。

在示例中，将错误映射到 `HttpResponse`，然后再使用 `?` 运算符。但如果你返回的错误类型实现了 [`ResponseError`][response-error] trait，则不需要执行此操作。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/databases/src/main.rs:index}}
```

请参见此处的完整示例：https://github.com/actix/examples/tree/master/diesel

[web-block]: https://docs.rs/actix-web/3/actix_web/web/fn.block.html
[response-error]: https://docs.rs/actix-web/3/actix_web/trait.ResponseError.html
