# URL 调度

> [url-dispatch.md](https://github.com/actix/actix-website/blob/master/content/docs/url-dispatch.md)
> <br />
> commit - e7ca7fddd7e642b985921f276bb4c53e5fd20a4e - 2020.09.21

URL 调度提供了简便的方式以进行简单的模式匹配，将 URL 映射到 handler 代码。如果请求相关联的路径信息与一个模式匹配，则调用特定的 handler 对象。

> 请求 handler 是一个函数，它接受可以从请求中提取的零个或多个参数（即 [*impl FromRequest*][implfromrequest]），并返回可以转换为 HttpResponse（即 [*impl Responder*][implresponder]）的类型。更多信息请查阅 [handler 章节](./handlers.md)。

# 资源配置

资源配置的作用是向应用程序添加新的资源。资源有用于 URL 生成的标识符作为名称，该名称允许开发人员向现有资源添加路由。资源也具有模式，用于匹配 *URL* 的部分*路径（PATH）*，即 scheme 和 port 之后的路径部分（如 *URL http://localhost:8080/foo/bar?q=value* 中的 */foo/bar* 部分）。但不匹配路径中的*查询（QUERY）*，即 *?* 后面的部分（如 *http://localhost:8080/foo/bar?q=value* 中的 *q=value*）。

[*App::route()*][approute] 方法提供了简便的方式以便于注册路由。此方法用于向应用程序的路由表添加路由，并且接受*路径模式*、*HTTP 方法*，以及 handler 函数。对于相同的资源路径，可以多次调用 `route()` 方法，在这种情况下，多个路由注册为同一个资源路径。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/main.rs:main}}
```

尽管 *App::route()* 方法提供了简便的方式以便于注册路由，但要访问完整的资源配置，必须使用不同的方法。[*App::service()*][appservice] 方法将单个[资源][webresource]添加到应用程序的路由表中，该方法接受*路径模式*、卫语句（guards），以及一个或多个路由。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/resource.rs:resource}}
```

如果资源不包含任何路由，或没有任何匹配的路由，将返回 *NOT FOUND* HTTP 响应。

## 路由配置

资源包含一系列路由。每个路由依次有一组 `卫语句（guards）` 和一个 handler。可以使用 `Resource::route()` 方法创建新路由，该方法返回对新路由 *Route* 实例的引用。默认情况下，路由不包含任何卫语句，因此可以匹配所有请求，其默认 handler 为 `HttpNotFound`。

基于资源注册与路由注册期间定义的路由标准，应用程序的路由传入请求。
应用程序根据在资源注册和路由注册期间定义的路由条件路由传入的请求。根据 `Resource::route()` 方法注册路由的顺序，资源匹配它包含的所有路由。

> 路由 *Route* 可含有任意多的*卫语句（guards）*， 但只能有一个 handler。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/cfg.rs:cfg}}
```

在这个示例中，如果 *GET* 请求包含 `Content-Type` 消息标头，则返回 `HttpResponse::Ok()`。此消息标头的值为 *text/plain*，路径为 `/path`。

如果资源无法匹配任何路由，则返回 "NOT FOUND" 响应。

[*ResourceHandler::route()*][resourcehandler] 返回[*路由（Route）*][route]对象。可以使用类似于 builder 的模式来配置路由。可用配置方法如下：

* [*Route::guard()*][routeguard] 注册一个新的卫语句（guard），每个路由可以注册任意数量的卫语句（guard）。
* [*Route::method()*][routemethod] 注册一个方法作为卫语句（guard），每个路由可以注册任意数量的卫语句（guard）。
* [*Route::to()*][routeto] 为路由注册一个异步 handler 函数，仅能注册一个 handler。通常 handler 注册是最后一个配置操作。

# 路由匹配

路由配置的主要目的是根据 URL 路径模式去匹配（或不匹配）请求的`路径（path）`。`路径（path）`表示被请求 URL 的路径部分。

*actix-web* 中，路由配置的方法非常简单。当请求进入系统时，将自身向系统中的每个资源配置予以声明，actix 会根据声明的模式去检查请求的路径。根据 `App::service()` 方法所声明路由的顺序，检查工作依次进行。如果未找到资源，则匹配的资源为*默认资源*。

当路由配置被声明时，可以包含路由卫语句参数。在检查期间，对于给定请求的路由配置来说，其与路由声明关联的所有路由卫语句都必须为 `true`。在检查期间，如果在提供给路由配置的路由卫语句参数集合中，有任意一个卫语句返回 `false`，则跳过该路由。然后，根据有序的路由集合，路由匹配将继续进行。

如果匹配到任何路由，则停止路由匹配进程，并调用与该路由关联的 handler。如果在用尽所有路由模式后，仍然没有路由匹配，则返回 *NOT FOUND* 响应。

# 资源模式语法

在模式参数匹配中，actix 使用的模式匹配语法简单明确。

在路由配置中，使用的模式可以以斜杠字符 `/` 开头。如果模式不是以斜杠字符 `/` 开头，匹配时则会在其前面加上一个隐式斜杠。例如，以下模式是等效的：

```
{foo}/bar/baz
```

以及：

```
/{foo}/bar/baz
```

*可变部分*（替换标记）以 *{id}* 的形式指定，这意味着——下一个斜杠字符 `/` 之前，接受任意字符，并将其用作 `HttpRequest.match_info()` 对象的名称。

模式中的替换标记，匹配正则表达式 `[^{}/]+`。

匹配信息（match_info）是 `Params` 对象，表示以路由模式为依据，从 *URL* 中提取的动态部分。匹配信息（match_info）也可以作为请求的匹配信息，如 *request.match_info*。下面示例模式中，定义了一个文本段（foo）和两个替换标记（baz 和 bar）：

```
foo/{baz}/{bar}
```

此模式将匹配如下 URL，可生成以下匹配信息：

```
foo/1/2        -> Params {'baz':'1', 'bar':'2'}
foo/abc/def    -> Params {'baz':'abc', 'bar':'def'}
```

但是，下述模式不会被匹配：

```
foo/1/2/        -> No match (trailing slash)
bar/abc/def     -> First segment literal mismatch
```

在路径段正则模式中，替换标记仅匹配到路径段中的第一个非字母数字字符。例如，如果使用这种路由模式：

```
foo/{name}.html
```

文本路径 */foo/biz.html* 将匹配上面的路由模式，匹配结果为 `Params{'name': 'biz'}`。但是，文本路径 */foo/biz* 不会匹配，因为末尾未包含 *.html* 字段。

如果两种文本路径都要匹配，可以使用两个替换标记：

```
foo/{name}.{ext}
```

文本路径 */foo/biz.html* 将匹配上面的路由模式，匹配结果为 *Params{'name': 'biz', 'ext': 'html'}*。这样写是因为在替换标记 *{name}* 和 *{ext}* 之间，存在一个文本部分 *.（点号）*。

替换标记可以可选地指定一个正则表达式，该表达式将用于决定路径段是否应与替换标记匹配。要指定替换标记仅匹配正则表达式定义的特定字符集，必须对替换标记语法做一些形式上的扩展。在大括号 `{}` 中，替换标记名称后，必须跟随冒号 `:`，然后是正则表达式。与替换标记 *[^/]+* 关联的默认正则表达式，可匹配一个或多个非斜杠字符。例如，底层的替换标记 *{foo}* 可以更详细地写为 *{foo:[^/]+}*。你可以将此更改为具体的正则表达式，以匹配具体的字符序列。比如更改为 *{foo:\d+}*，将仅匹配数字。

路径段必须至少包含一个字符，才能匹配路径的替换标记。例如，对于 URL 路径 */abc/*：

* */abc/{foo}* 不会匹配；
* */{foo}/* 可以匹配。

> **注意**：在匹配模式前，将对 URL 路径去除引号，并解码为有效的 unicode 字符串；且代表路径段的匹配值，也将是去除引号的 URL。

例如，对于如下模式：

```
foo/{bar}
```

在匹配如下 URL 时：

```
http://example.com/foo/La%20Pe%C3%B1a
```

匹配字典如下所示（URL 解码后的值)：

```
Params{'bar': 'La Pe\xf1a'}
```

路径段中的文本字符串代表路径的解码值，以提供给 actix。你不会希望在模式中使用 URL 编码值。例如，不是这样的 URL 编码值：

```
/Foo%20Bar/{baz}
```

你会希望使用这样的值：

```
/Foo Bar/{baz}
```

但这样做有可能得到“尾部匹配（tail match）”，为此，必须使用自定义正则表达式。

```
foo/{bar}/{tail:.*}
```

上述模式可匹配如下 URL，并生成如下匹配信息：

```
foo/1/2/           -> Params{'bar':'1', 'tail': '2/'}
foo/abc/def/a/b/c  -> Params{'bar':u'abc', 'tail': 'def/a/b/c'}
```

# 作用域路由

作用域可以帮助你组织路由，以共享共用的根路径。作用域可以嵌套。

比如，你希望组织一组路径，用于查看 "Users" 端资源。这些路径可能包括：

- /users
- /users/show
- /users/show/{id}

这些路径的作用域布局如下所示：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/scope.rs:scope}}
```

*作用域* 路径可以包含可变路径段，与非作用域路径用法一致。

你可以使用 `HttpRequest::match_info()` 方法获取可变路径段，[`Path` 提取器](./extractors.md)也可以提取作用域层级的变量段。

# 匹配信息

所有代表路径段的匹配值，都可以使用 [`HttpRequest::match_info`][matchinfo] 方法获得。[`Path::get()`][pathget] 方法可用于检索特定值。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/minfo.rs:minfo}}
```

示例中的路径 '/a/1/2/'，其中的 v1 和 v2 两个值将被解析为 “1” 和 “2”。

可以由路径尾部的参数创建 `PathBuf`，`PathBuf` 返回值经百分比解码（URL 解码）。如果分段是 `..`，则跳过前一个分段（如果存在）。

出于安全目的，如果分段满足以下任一条件，则返回一个 `Err`，表示该条件已满足：

* 解码段的开头为（任一）：`.`（不包括 `..`），`*`
* 解码段的结尾为（任一）：`:`，`>`，`<`
* 解码段包含（任一）：`/`
* Windows 环境，解码段包含（任一）：`\`
* 百分比编码（URL 编码）导致的无效 UTF8。

基于上述条件，从请求路径参数解析的 `PathBuf`，可以安全地在路径内插入，或用作路径的后缀，而无需额外检查。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/pbuf.rs:pbuf}}
```

## 路径信息提取

actix 提供类型安全的路径信息提取的功能。使用 [*Path*][pathstruct] 结构体提取路径信息后，目标类型可以定义为几种不同的形式。最简单的方式是使用`元组（tuple）`类型，元组中的每个元素必须对应于路径模式中的一个元素。也就是说，你可以将路径模式 `/{id}/{username}/` 与类型 `Path<(u32, String)>` 成功匹配，但是与类型 `Path<(String, String, String)>` 的匹配就会失败。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/path.rs:path}}
```

也可以将路径模式信息提取到结构体中。下述示例中，结构体必须反序列化，实现 *serde* crate 的 `Deserialize` trait。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/path2.rs:path}}
```

[*Query*][query] 结构体为请求查询参数提供了类似的功能。

# 生成资源 URL

使用 [*HttpRequest.url_for()*][urlfor] 方法，生成基于资源模式的 URL。例如，如果您配置了一个名称为“foo”，且模式为“{a}/{b}/{c}”的资源，则可以执行以下操作：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/urls.rs:url}}
```

这会返回类似 *http://example.com/test/1/2/3* 的字符串（协议和主机名仅为示例）。`url_for()` 返回结构体 [*Url 对象*][urlobj]，你可以对 url 进行修改（添加查询参数、锚点等）。只有`已命名`资源调用可以 `url_for()` 方法，否则返回错误。

# 外部资源

有效的资源 URL，可以注册为外部资源。外部资源仅用于生成 URL，在请求时，从不考虑进行匹配。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/url_ext.rs:ext}}
```

# 路径规范化，以及重定向到附加斜杠的路由

路径规范化意味着：

* 对路径附加尾部斜杠；
* 规范路径中的斜杠，用一个斜杠替换连续的多个斜杠。

路径规范化处理程序一旦找到正确解析的路径，就会立刻返回。如果启用了所有规范化条件，则其顺序为：1）合并，2）合并和追加，以及 3）追加。如果路径至少在其中一个条件下解析，它将重定向到新路径。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/norm.rs:norm}}
```

示例中，`//resource///` 将会被重定向为 `/resource/`。

上述示例中，为所有方法都注册了路径规范化处理程序，但你不应依赖于这种机制去重定向 *POST* 请求。附加斜杠的 *Not Found* 路径，其重定向会丢失原始请求中的所有 *POST* 数据，将 *POST* 请求转换为 GET 请求。

可以仅对 *GET* 请求注册路径规范化处理程序：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/norm2.rs:norm}}
```

## 使用作用域前缀组合应用

`web::scope()` 方法允许设置特定的应用程序作用域。此作用域表示一个资源前缀，该前缀将预置到由资源配置添加的所有资源模式中。这可以用来帮助装载一组路由到新的 URL 路径，而与其包含的可调用 URL 路径不同，同时仍保持相同的资源名称。

例如：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/scope.rs:scope}}
```

在上面的示例中，*show_users* 路由将具有有效路由模式 */users/show*，而不是 */show*，因为应用程序作用域将预先添加到路由模式中。只有当 URL 路径匹配 */users/show*，并且使用路由名称 `show_users` 调用 `HttpRequest.url_for()` 函数时，它将生成具有相同路径的 URL。

# 自定义路由卫语句

可以将卫语句视作为一个简单的函数，它接受*请求* 对象引用，并返回 *true* 或 *false*。从形式上讲，卫语句是实现 [`Guard`][guardtrait] trait 的任何对象。actix 提供了几个断言，详细了解请可以查看 API 文档的[函数章节][guardfuncs]。

下面示例是一个简单的卫语句，用于检查请求是否包含特定的*消息标头*：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/guard.rs:guard}}
```

上述示例中，只有当请求包含 *CONTENT-TYPE* 消息标头时，才会调用*index* handler。

卫语句不能访问或修改请求对象，但是可以在[请求扩展][requestextensions]中存储额外的信息。

## 修改卫语句的值

通过将断言值包裹在 `Not` 断言中，可以反转任何断言值的含义。例如，如果要为除 `GET` 之外的所有方法返回 `METHOD NOT ALLOWED` 响应：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/guard2.rs:guard2}}
```

如果要匹配所提供卫语句列表中的任意一个，可以使用 `Any` 卫语句。即：

```rust
guard::Any(guard::Get()).or(guard::Post())
```

如果要匹配所提供的卫语句列表中的全部项，可以使用 `All` 卫语句。即：

```rust
guard::All(guard::Get()).and(guard::Header("content-type", "plain/text"))
```

# 更改默认的 `Not Found` 响应

如果在路由表中不能发现路径模式，或资源找不到可匹配的路由，则会使用默认资源。默认的响应是 *NOT FOUND*，我们可以使用 `App::default_service()` 方法重写 *NOT FOUND* 响应。此方法通过 `App::service()` 方法接受*配置函数*，与普通资源配置方法相同。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/dhandler.rs:default}}
```

[approute]: https://docs.rs/actix-web/3/actix_web/struct.App.html#method.route
[appservice]: https://docs.rs/actix-web/3/actix_web/struct.App.html?search=#method.service
[webresource]: https://docs.rs/actix-web/3/actix_web/struct.Resource.html
[resourcehandler]: https://docs.rs/actix-web/3/actix_web/struct.Resource.html#method.route
[route]: https://docs.rs/actix-web/3/actix_web/struct.Route.html
[routeguard]: https://docs.rs/actix-web/3/actix_web/struct.Route.html#method.guard
[routemethod]: https://docs.rs/actix-web/3/actix_web/struct.Route.html#method.method
[routeto]: https://docs.rs/actix-web/3/actix_web/struct.Route.html#method.to
[matchinfo]: https://docs.rs/actix-web/3/actix_web/struct.HttpRequest.html#method.match_info
[pathget]: https://docs.rs/actix-web/3/actix_web/dev/struct.Path.html#method.get
[pathstruct]: https://docs.rs/actix-web/3/actix_web/dev/struct.Path.html
[query]: https://docs.rs/actix-web/3/actix_web/web/struct.Query.html
[urlfor]: https://docs.rs/actix-web/3/actix_web/struct.HttpRequest.html#method.url_for
[urlobj]: https://docs.rs/url/1.7.2/url/struct.Url.html
[guardtrait]: https://docs.rs/actix-web/3/actix_web/guard/trait.Guard.html
[guardfuncs]: https://docs.rs/actix-web/3/actix_web/guard/index.html#functions
[requestextensions]: https://docs.rs/actix-web/3/actix_web/struct.HttpRequest.html#method.extensions
[implfromrequest]: https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html
[implresponder]: https://docs.rs/actix-web/3/actix_web/trait.Responder.html
