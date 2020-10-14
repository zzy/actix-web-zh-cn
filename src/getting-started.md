# 入门指南

> [getting-started.md](https://github.com/actix/actix-website/blob/master/content/docs/getting-started.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

## 安装 Rust

如果你还未安装 Rust，推荐使用 `rustup` 来管理你的 Rust，详细参见[官方的 Rust 安装指南][rustguide]。

actix-web 目前支持的最低支持为 Rust-1.42（MSRV），运行 `rustup update` 将更新到 Rust 的最新版本。本指南假设您运行的是 Rust-1.42 或更高版本。

## 创建项目

首先，创建一个新的二进制 Cargo 项目，并切换到新目录：

```bash
cargo new hello-world
cd hello-world
```

通过向 `Cargo.toml` 文件添加以下内容，将 `actix-web` 添加为项目的依赖项。

```toml
[dependencies]
actix-web = "3"
```

请求处理程序使用异步函数，接受零个或多个参数。这些参数可以从请求中提取（参见 `FromRequest` trait），并返回可以转换为 `HttpResponse` 的类型（参见 `Responder` trait）：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/getting-started/src/main.rs:handlers}}
```

请注意，其中一些处理程序因为使用内建宏的原因，直接附加了路由信息，从而允许你指定处理程序应该响应的方法和路径。你将在下面示例看到不使用路由宏的情况下，如何注册自定义路由。

接下来，创建 `App` 实例并注册请求处理程序。对于使用了路由宏的处理程序，使用 `App::service` 方法注册路由；对不使用路由宏而注册自定义路由的情况，使用 `App::route` 方法。最后，使用 `HttpServer` 启动应用程序，它将你的 `App` 实例作为“应用程序工厂”，以处理传入的请求。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/getting-started/src/main.rs:main}}
```

代码工作基本完成，我们使用 `cargo run` 编译并运行程序。在 actix 运行时中，`#[actix_web::main]` 宏用来执行异步 main 函数。现在，你可以访问 `http://localhost:8080/` 或你定义的任何其他路由，以查看运行结果。

<!-- LINKS -->

[rustguide]: https://rust-lang.budshome.com/ch01-01-installation.html
[actix-web-codegen]: https://docs.rs/actix-web-codegen/
