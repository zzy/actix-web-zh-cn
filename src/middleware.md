# 中间件

> [middleware.md](https://github.com/actix/actix-website/blob/master/content/docs/middleware.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

actix-web 的中间件系统允许我们在请求/响应处理中添加额外的行为。中间件可以钩入（hook into）到传入的请求进程中，使我们能够修改请求以及暂停请求处理，以尽早返回响应。

中间件也可以钩入（hook into）到响应处理进程中。

典型地，中间件与以下操作相关联：

* 请求预处理
* 响应后置处理
* 修改应用状态
* 访问扩展服务（redis、日志、会话）

每个`应用（App）`、`作用域（scope）`，或者`资源（Resource）`都可以注册中间件，并按照与注册顺序相反的过程执行。通常，*中间件* 是一种实现了 [*Service trait*][servicetrait] 和 [*Transform trait*][transformtrait] 的类型。在 [*Service trait*][servicetrait] 和 [*Transform trait*][transformtrait] 中，每个方法都有一个默认实现，每个方法都可以立即返回结果或返回 *future* 对象。

下面演示如何创建一个简单的中间件：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/middleware/src/main.rs:simple}}
```

或者，对于简单的用例，你可以使用 [*wrap_fn*][wrap_fn] 来创建小型的临时中间件：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/middleware/src/wrap_fn.rs:wrap-fn}}
```

> actix-web 提供了一些有用的中间件，比如*日志*、*用户会话*、*压缩* 等。

## 日志

actix-web 中，日志是作为中间件实现的。通常，将日志中间件注册为应用程序的第一个中间件。并且，必须为每个应用程序注册日志中间件。

`Logger` 中间件使用标准的日志 crate 来记录日志信息。你应该为 *actix_web* 包启用 logger，以查看访问日志（[env_logger][envlogger] 或类似 crate）。

### 使用方法

使用指定`格式`创建 `Logger` 中间件。可以使用 `default` 方法创建默认 `Logger`，默认格式为：

```ignore
  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
```

```rust,edition2018,no_run,noplaypen
{{#include ../examples/middleware/src/logger.rs:logger}}
```

下面是默认日志记录格式的示例：

```
INFO:actix_web::middleware::logger: 127.0.0.1:59934 [02/Dec/2017:00:21:43 -0800] "GET / HTTP/1.1" 302 0 "-" "curl/7.54.0" 0.000397
INFO:actix_web::middleware::logger: 127.0.0.1:59947 [02/Dec/2017:00:22:40 -0800] "GET /index.html HTTP/1.1" 200 0 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:57.0) Gecko/20100101 Firefox/57.0" 0.000646
```

### 格式

- `%%`  百分号
- `%a`  远程 IP 地址（若使用了反向代理，则为代理地址）
- `%t`  请求开始处理的时间
- `%P`  请求的子服务进程 ID
- `%r`  请求的第一行
- `%s`  响应状态码
- `%b`  包含 HTTP 消息标头的响应字节大小
- `%T`  请求服务所用时间, 单位为秒，格式为浮点分数（.06f）
- `%D`  请求服务所用时间, 单位为毫秒
- `%{FOO}i`  request.headers['FOO']
- `%{FOO}o`  response.headers['FOO']
- `%{FOO}e`  os.environ['FOO']

## 默认消息标头

要设置默认的响应消息标头，可以使用 `DefaultHeaders` 中间件。如果响应消息已包含指定标头，则 *DefaultHeaders* 中间件不会再设置。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/middleware/src/default_headers.rs:default-headers}}
```

## 用户会话

actix-web 为会话管理提供了一个通用的解决方案。[**actix-session**][actixsession] 中间件可以使用多种后端类型来存储会话数据。

> 默认情况下，只实现了 cookie 会话后端，但可以添加其它后端实现。

[**CookieSession**][cookiesession] 使用 cookie 作为会话存储。因为有效负载必须适合单个 cookie，`CookieSessionBackend` 创建的会话仅限于存储少于 4000 字节的数据。如果会话包含超过 4000 个字节，则会产生内部服务器错误。

cookie 的安全策略方面，可以是*签名的*，或*私有的*。各自有其 `CookieSession` 构造函数。

客户端可以查看*已签名* cookie，但不能对其进行修改。客户端既不能查看也不能修改*私有* cookie。

构造函数接受一个键作为参数，作为 cookie 会话的私钥——更改此值时，所有会话数据都将丢失。

一般来说，创建一个 `SessionStorage` 中间件，并使用特定的后端实现（如 `CookieSession`）对其初始化。要访问会话数据，必须使用 [`Session`][requestsession] 提取器。这个方法返回一个 [*Session*][sessionobj] 对象，它允许我们获取或设置会话数据。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/middleware/src/user_sessions.rs:user-session}}
```

## 错误处理

`ErrorHandlers` 中间件允许我们为响应提供自定义处理程序。

可以使用 `ErrorHandlers::handler()` 方法为特定状态代码注册自定义错误处理程序。你可以修改已存在的响应，或创建全新的响应。错误处理程序可以立即返回响应，也可以返回解析为响应的 future。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/middleware/src/errorhandler.rs:error-handler}}
```

[sessionobj]: https://docs.rs/actix-session/0.3.0/actix_session/struct.Session.html
[requestsession]: https://docs.rs/actix-session/0.3.0/actix_session/struct.Session.html
[cookiesession]: https://docs.rs/actix-session/0.3.0/actix_session/struct.CookieSession.html
[actixsession]: https://docs.rs/actix-session/0.3.0/actix_session/
[envlogger]: https://docs.rs/env_logger/*/env_logger/
[servicetrait]: https://docs.rs/actix-web/3/actix_web/dev/trait.Service.html
[transformtrait]: https://docs.rs/actix-web/3/actix_web/dev/trait.Transform.html
[wrap_fn]: https://docs.rs/actix-web/3/actix_web/struct.App.html#method.wrap_fn
