# HTTP 服务器

> [server.md](https://github.com/actix/actix-website/blob/master/content/docs/server.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

[**HttpServer**][httpserverstruct] 类型负责为 HTTP 请求提供服务。

`HttpServer` 接受应用程序工厂作为参数，并且应用程序工厂必须具有 `Send` + `Sync` 约束。在[“多线程”](#多线程)一节有更多信息。

- 要绑定到特定的套接字（socket）地址，必须使用 [`bind()`][bindmethod] 方法，并且可以多次调用它。
- 要绑定 ssl 套接字（socket），应使用 [`bind_openssl()`][bindopensslmethod] 方法或者
[`bind_rustls()`][bindrusttls] 方法。
- 要运行 HTTP 服务器，请使用 `HttpServer::run()` 方法。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/server/src/main.rs:main}}
```

`run()` 方法返回 [`Server`][server] 类型的实例，`Server` 类型的方法可用于管理 HTTP 服务器：

- `pause()` - 暂停接受传入的连接
- `resume()` - 重新开始接受传入的连接
- `stop()` - 停止处理传入的连接，停止所有工作线程，并退出

下面的示例展示了如何在独立的线程中启动 HTTP 服务器。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/server/src/signals.rs:signals}}
```

## 多线程

`HttpServer` 会自动启动多个 HTTP 工作线程（worker），默认情况下，线程的数量等于系统中逻辑 CPU 的数量。线程的数量可以用 [`HttpServer::workers()`][workers] 方法重写。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/server/src/workers.rs:workers}}
```

一旦创建了线程（worker），每个线程（worker）都会收到一个独立的 App（应用程序）实例，每个 App 实例都可以处理请求。应用程序状态（state）在线程之间不能共享，但处理程序可以自由地操作其状态副本，而无需担心并发性问题。

> 应用程序状态（state）不需要具有 `Send` 或者 `Sync` 约束，但应用程序工厂必须具有 `Send` + `Sync` 约束。

要在工作线程之间共享状态，要使用 `Arc（原子引用计数器）`。一旦引入共享和同步，应格外小心。在许多情况下，由于锁定共享状态以对其进行修改，会无意中引入性能成本。

在某些情况下，使用更有效的锁定策略可以降低这些性能成本。例如，使用[读/写锁](https://doc.rust-lang.org/std/sync/struct.RwLock.html)而不是[互斥器（mutexes）](https://doc.rust-lang.org/std/sync/struct.Mutex.html)来实现非互斥锁，但最具性能的实现，往往是根本不发生锁定的实现。

由于每个工作线程都是按顺序处理其请求的，因此当处理程序阻塞当前线程时，将导致当前工作线程停止处理新请求：

```rust,edition2018,no_run,noplaypen
fn my_handler() -> impl Responder {
    std::thread::sleep(Duration::from_secs(5)); // <-- Bad practice! Will cause the current worker thread to hang!
    "response"
}
```

因上述原因，任何长时间的、非 cpu 限制的操作（例如，I/O、数据库操作等）都应该使用 `future` 函数或异步函数。异步处理程序由工作线程并发执行，因此不会发生阻塞：

```rust,edition2018,no_run,noplaypen
async fn my_handler() -> impl Responder {
    tokio::time::delay_for(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    "response"
}
```

同样的限制也适用于提取器（extractor）。当处理程序函数接收到实现了 `FromRequest` 的参数，并且该实现阻塞当前线程时，工作线程也将在运行时发生阻塞。因此，在实现提取器（extractor）时必须特别注意，而且在需要时也应该异步地实现它们。

## SSL

ssl 服务器有两个实现：`rustls` 和 `openssl`。`rustls` 集成在 Rust 程序设计语言新开发的 TLS 库 `rustls`，`openssl` 用于开源的 TLS 业界标准库 `openssl`。

```toml
[dependencies]
actix-web = { version = "{{< actix-version "actix-web" >}}", features = ["openssl"] }
openssl = { version="0.10" }
```

```rust,edition2018,no_run,noplaypen
{{#include ../examples/server/src/ssl.rs:ssl}}
```

> **注意**：*HTTP/2.0* 需要 [tls alpn][tlsalpn] 协议支持。目前，仅有 `openssl` 支持 `alpn` 协议。完整实例请查看 [examples/openssl][exampleopenssl]。

使用命令创建 key.pem 和 cert.pem，注意**填写你自己的主题**：

```bash
$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
  -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
```

移除密码，然后将 nopass.pem 复制到 key.pem：

```bash
$ openssl rsa -in key.pem -out nopass.pem
```

## Keep-Alive

actix 可以在 keep-alive 连接上等待请求。

> *keep-alive* 连接行为由服务器设置定义：

- `75`、`Some(75)`、`KeepAlive::Timeout(75)` - 开启 *keep-alive* 保活时间为 75 秒。
- `None` 或者 `KeepAlive::Disabled` - 关闭 *keep-alive*。
- `KeepAlive::Tcp(75)` - 使用 `SO_KEEPALIVE` 套接字选项。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/server/src/keep_alive.rs:keep-alive}}
```

如果选择了上面的第一个选项，则根据响应的 *connection-type* 计算 *keep alive* 状态。默认情况下，`HttpResponse::connection_type` 是未定义的，*keep-alive* 状态由请求的 HTTP 版本定义。

> 对于 *HTTP/1.0*，*keep-alive* 默认**关闭**；对于 *HTTP/1.1* 和 *HTTP/2.0*，*keep-alive* 默认**开启**。

可以使用 `HttpResponseBuilder::connection_type()` 方法更改 *connection-type*。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/server/src/keep_alive_tp.rs:example}}
```

## 正常关闭

`HttpServer` 支持正常关闭。在收到停止信号后，工作线程分配一定的时间来完成服务请求。超时后，任何仍然存活的工作线程都将被强制丢弃。默认情况下，关机超时设置为 30 秒，可以使用 [`HttpServer::shutdown_timeout()`][shutdowntimeout] 方法更改此参数。

你可以通过服务器地址向服务器发送停止消息，并指明是否要正常关闭。`Server` 类型有 [`start()`][startmethod] 方法，可以返回服务器地址。

`HttpServer` 会处理多个操作系统信号。其中 *CTRL-C* 可用于所有操作系统，其他信号可用于 unix 系统。

- *SIGINT* - 强制关闭工作线程
- *SIGTERM* - 正常关闭工作线程
- *SIGQUIT* - 强制关闭工作线程

> 使用 [`HttpServer::disable_signals()`][disablesignals] 方法可以禁用信号处理。

[server]: https://docs.rs/actix-web/3/actix_web/dev/struct.Server.html
[httpserverstruct]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html
[bindmethod]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind
[bindopensslmethod]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind_openssl
[bindrusttls]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind_rustls
[startmethod]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.start
[workers]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.workers
[tlsalpn]: https://tools.ietf.org/html/rfc7301
[exampleopenssl]: https://github.com/actix/examples/blob/master/openssl
[shutdowntimeout]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.shutdown_timeout
[disablesignals]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.disable_signals
