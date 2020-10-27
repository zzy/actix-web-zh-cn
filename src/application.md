# 编写应用程序

> [application.md](https://github.com/actix/actix-website/blob/master/content/docs/application.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

`actix-web` 提供了各种原语，以使用 Rust 程序设计语言构建 web 服务器和应用程序。它提供路由、中间件、预处理请求，以及响应的后置处理等。

所有 `actix-web` 服务器都是围绕 [`App`][app]（应用程序）实例构建的，其用于注册资源路由和中间件。另外，在同一作用域（scope）中，`actix-web` 服务器也存储所有处理程序之间共享的应用程序状态。

应用程序的[`作用域（scope）`][scope]充当所有路由的命名空间，也就是说，特定的应用程序作用域内，所有路由都具有相同的 url 路径前缀。应用程序的路由前缀始终包含前导斜杠 “/”；如果提供的前缀不包含前导斜杠 “/”，则会自动补入该前缀。除了前导斜杠 “/”，路由前缀也应该包含路径值。

> 比如，应用程序的作用域为 `/app`，即路径前缀为 `/app`。那么，路径为 `/app`、`/app/`，或者 `/app/test` 的请求都可以匹配；但是，路径 `/application` 不能匹配。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/application/src/app.rs:setup}}
```

上述例子中，创建了具有 `/app` 前缀和 `index.html` 页面资源的应用程序。此资源可通过 url 路径 `/app/index.html` 获得。

> 要获取更多信息，请参阅 [URL 调度 - 使用作用域前缀][usingappprefix]一节。

## 状态（state）

应用程序状态（state）被同一作用域（scope）内的所有路由和资源共享。可以使用数据提取器 [`web::Data<T>`][data] 访问状态（state），其中泛型参数 `T` 表示状态类型。另外，中间件也可以访问状态。

让我们编写一个简单的应用程序，并将应用程序名称存储在状态中：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/application/src/state.rs:setup}}
```

并在 App 初始化时传入状态（state），然后启动应用程序：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/application/src/state.rs:start_app}}
```

在应用程序中，可以注册任意数量的状态（state）类型。

## 共享可变状态

`HttpServer` 接受应用程序工厂，而非应用程序实例。`HttpServer` 为每个线程构造一个应用程序实例。因此，必须多次构造应用程序数据。如果你想在不同的线程之间共享数据，应该使用一个可共享的对象，例如 `Send` + `Sync`。

[`web::Data`][data] 内部使用 `Arc（原子引用计数器）`。因此，为了避免创建两个 `Arc（原子引用计数器）`，我们应该在使用 [`App::app_data()`][appdata] 方法注册数据之前，先行创建数据。

在下面的示例中，我们将编写一个应用程序，其具有可变的、共享的状态（state）。首先，我们定义状态并创建处理程序：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/application/src/mutable_state.rs:setup_mutable}}
```

然后，在 `App` 中注册数据：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/application/src/mutable_state.rs:make_app_mutable}}
```

## 使用作用域组合应用程序

[`web::scope()`][webscope] 方法允许设置资源组前缀。此作用域表示一个预添加的资源前缀——在由资源配置添加的所有资源模式中，该前缀将被预先附加。这有助于将一组新编写的路由挂载到不同位置，从而与以前开发者设计的位置分离，但仍然保持相同的资源名称。 

我们来看实际例子：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/application/src/scope.rs:scope}}
```

在上面的示例中，`show_users` 路由的有效路由模式将是 `/users/show`，而非 `/show`，因为应用程序的 `scope` 参数将附加在模式前面。仅当 URL 路径为 `/users/show` 时，路由才将匹配。使用路由名称 `show_users` 调用函数 [`HttpRequest.url_for()`][urlfor]，它将生成具有相同路径的 URL。

## 应用程序卫语句及虚拟主机

可以将卫语句看作是一个简单的函数，它接受 _request_ 对象引用，并返回
_true_ 或者 _false_。从形式上讲，卫语句是实现了 [`Guard`][guardtrait] trait 的任何对象。actix-web 提供了多种卫语句，要详细了解，请查看 API 文档的[函数部分][guardfuncs]。

[`Header`][guardheader] 是 actix-web 提供的卫语句之一，它可以用作基于请求头信息的过滤器。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/application/src/vh.rs:vh}}
```

# 配置

为了简洁和可重用，[`App`][appconfig] 和 [`web::Scope`][webscopeconfig] 均提供了 `configure` 方法，此函数用于将配置的部分移动到不同的模块甚至库中。例如，资源的某些配置可以移动到其它模块。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/application/src/config.rs:config}}
```

上述示例的结果是：

```
/         -> "/"
/app      -> "app"
/api/test -> "test"
```

每一个 [`ServiceConfig`][serviceconfig] 可以有自己的`数据（data）`、`路由（route）`，以及`服务（services）`。

<!-- LINKS -->

[usingappprefix]: ./url-dispatch.md#使用作用域前缀
[stateexample]: https://github.com/actix/examples/blob/master/state/src/main.rs
[guardtrait]: https://docs.rs/actix-web/3/actix_web/guard/trait.Guard.html
[guardfuncs]: https://docs.rs/actix-web/3/actix_web/guard/index.html#functions
[guardheader]: https://docs.rs/actix-web/3/actix_web/guard/fn.Header.html
[data]: https://docs.rs/actix-web/3/actix_web/web/struct.Data.html
[app]: https://docs.rs/actix-web/3/actix_web/struct.App.html
[appconfig]: https://docs.rs/actix-web/3/actix_web/struct.App.html#method.configure
[appdata]: https://docs.rs/actix-web/3/actix_web/struct.App.html#method.app_data
[scope]: https://docs.rs/actix-web/3/actix_web/struct.Scope.html
[webscopeconfig]: https://docs.rs/actix-web/3/actix_web/struct.Scope.html#method.configure
[webscope]: https://docs.rs/actix-web/3/actix_web/web/fn.scope.html
[urlfor]: https://docs.rs/actix-web/3/actix_web/struct.HttpRequest.html#method.url_for
[serviceconfig]: https://docs.rs/actix-web/3/actix_web/web/struct.ServiceConfig.html
