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

在这个示例中，如果 *GET* 请求包含 `Content-Type` 标头，则返回 `HttpResponse::Ok()`。此标头的值为 *text/plain*，路径为 `/path`。

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

# Resource pattern syntax

The syntax of the pattern matching language used by actix in the pattern
argument is straightforward.

The pattern used in route configuration may start with a slash character. If the pattern
does not start with a slash character, an implicit slash will be prepended
to it at matching time. For example, the following patterns are equivalent:

```
{foo}/bar/baz
```

and:

```
/{foo}/bar/baz
```

A *variable part* (replacement marker) is specified in the form *{identifier}*,
where this means "accept any characters up to the next slash character and use this
as the name in the `HttpRequest.match_info()` object".

A replacement marker in a pattern matches the regular expression `[^{}/]+`.

A match_info is the `Params` object representing the dynamic parts extracted from a
*URL* based on the routing pattern. It is available as *request.match_info*. For example, the
following pattern defines one literal segment (foo) and two replacement markers (baz, and bar):

```
foo/{baz}/{bar}
```

The above pattern will match these URLs, generating the following match information:

```
foo/1/2        -> Params {'baz':'1', 'bar':'2'}
foo/abc/def    -> Params {'baz':'abc', 'bar':'def'}
```

It will not match the following patterns however:

```
foo/1/2/        -> No match (trailing slash)
bar/abc/def     -> First segment literal mismatch
```

The match for a segment replacement marker in a segment will be done only up to
the first non-alphanumeric character in the segment in the pattern. So, for instance,
if this route pattern was used:

```
foo/{name}.html
```

The literal path */foo/biz.html* will match the above route pattern, and the match result
will be `Params{'name': 'biz'}`. However, the literal path */foo/biz* will not match,
because it does not contain a literal *.html* at the end of the segment represented
by *{name}.html* (it only contains biz, not biz.html).

To capture both segments, two replacement markers can be used:

```
foo/{name}.{ext}
```

The literal path */foo/biz.html* will match the above route pattern, and the match
result will be *Params{'name': 'biz', 'ext': 'html'}*. This occurs because there is a
literal part of *.* (period) between the two replacement markers *{name}* and *{ext}*.

Replacement markers can optionally specify a regular expression which will be used to decide
whether a path segment should match the marker. To specify that a replacement marker should
match only a specific set of characters as defined by a regular expression, you must use a
slightly extended form of replacement marker syntax. Within braces, the replacement marker
name must be followed by a colon, then directly thereafter, the regular expression. The default
regular expression associated with a replacement marker *[^/]+* matches one or more characters
which are not a slash. For example, under the hood, the replacement marker *{foo}* can more
verbosely be spelled as *{foo:[^/]+}*. You can change this to be an arbitrary regular expression
to match an arbitrary sequence of characters, such as *{foo:\d+}* to match only digits.

Segments must contain at least one character in order to match a segment replacement marker.
For example, for the URL */abc/*:

* */abc/{foo}* will not match.
* */{foo}/* will match.

> **Note**: path will be URL-unquoted and decoded into valid unicode string before
> matching pattern and values representing matched path segments will be URL-unquoted too.

So for instance, the following pattern:

```
foo/{bar}
```

When matching the following URL:

```
http://example.com/foo/La%20Pe%C3%B1a
```

The match dictionary will look like so (the value is URL-decoded):

```
Params{'bar': 'La Pe\xf1a'}
```

Literal strings in the path segment should represent the decoded value of the
path provided to actix. You don't want to use a URL-encoded value in the pattern.
For example, rather than this:

```
/Foo%20Bar/{baz}
```

You'll want to use something like this:

```
/Foo Bar/{baz}
```

It is possible to get "tail match". For this purpose custom regex has to be used.

```
foo/{bar}/{tail:.*}
```

The above pattern will match these URLs, generating the following match information:

```
foo/1/2/           -> Params{'bar':'1', 'tail': '2/'}
foo/abc/def/a/b/c  -> Params{'bar':u'abc', 'tail': 'def/a/b/c'}
```

# Scoping Routes

Scoping helps you organize routes sharing common root paths.  You can nest
scopes within scopes.

Suppose that you want to organize paths to endpoints used to view "Users". Such paths may include:

- /users
- /users/show
- /users/show/{id}


A scoped layout of these paths would appear as follows

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/scope.rs:scope}}
```

A *scoped* path can contain variable path segments as resources. Consistent with 
un-scoped paths.

You can get variable path segments from `HttpRequest::match_info()`.
[`Path` extractor](./extractors.md) also is able to extract scope level variable segments.

# Match information

All values representing matched path segments are available in [`HttpRequest::match_info`][matchinfo].
Specific values can be retrieved with [`Path::get()`][pathget].

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/minfo.rs:minfo}}
```

For this example for path '/a/1/2/', values v1 and v2 will resolve to "1" and "2".

It is possible to create a `PathBuf` from a tail path parameter. The returned `PathBuf` is
percent-decoded. If a segment is equal to "..", the previous segment (if
any) is skipped.

For security purposes, if a segment meets any of the following conditions,
an `Err` is returned indicating the condition met:

* Decoded segment starts with any of: `.` (except `..`), `*`
* Decoded segment ends with any of: `:`, `>`, `<`
* Decoded segment contains any of: `/`
* On Windows, decoded segment contains any of: '\'
* Percent-encoding results in invalid UTF8.

As a result of these conditions, a `PathBuf` parsed from request path parameter is
safe to interpolate within, or use as a suffix of, a path without additional checks.

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/pbuf.rs:pbuf}}
```

## Path information extractor

Actix provides functionality for type safe path information extraction.  [*Path*][pathstruct]
extracts information, destination type could be defined in several different forms. Simplest
approach is to use `tuple` type. Each element in tuple must correspond to one element from
path pattern. i.e. you can match path pattern `/{id}/{username}/` against
`Path<(u32, String)>` type, but `Path<(String, String, String)>` type will always fail.

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/path.rs:path}}
```

It also possible to extract path pattern information to a struct. In this case,
this struct must implement *serde's *`Deserialize` trait.

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/path2.rs:path}}
```

[*Query*][query] provides similar functionality for request query parameters.

# Generating resource URLs

Use the [*HttpRequest.url_for()*][urlfor] method to generate URLs based on resource
patterns. For example, if you've configured a resource with the name "foo" and the
pattern "{a}/{b}/{c}", you might do this:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/urls.rs:url}}
```

This would return something like the string *http://example.com/test/1/2/3* (at least if
the current protocol and hostname implied http://example.com).  `url_for()` method
returns [*Url object*][urlobj] so you can modify this url (add query parameters, anchor, etc).
`url_for()` could be called only for *named* resources otherwise error get returned.

# External resources

Resources that are valid URLs, can be registered as external resources. They are useful
for URL generation purposes only and are never considered for matching at request time.

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/url_ext.rs:ext}}
```

# Path normalization and redirecting to slash-appended routes

By normalizing it means:

* To add a trailing slash to the path.
* To replace multiple slashes with one.

The handler returns as soon as it finds a path that resolves correctly. The order of
normalization conditions, if all are enabled, is 1) merge, 2) both merge and append and
3) append. If the path resolves with at least one of those conditions, it will redirect
to the new path.

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/norm.rs:norm}}
```

In this example `//resource///` will be redirected to `/resource/`.

In this example, the path normalization handler is registered for all methods,
but you should not rely on this mechanism to redirect *POST* requests. The redirect of the
slash-appending *Not Found* will turn a *POST* request into a GET, losing any
*POST* data in the original request.

It is possible to register path normalization only for *GET* requests only:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/norm2.rs:norm}}
```

## 使用作用域前缀

The `web::scope()` method allows to set a specific application scope.  This scope represents
a resource prefix that will be prepended to all resource patterns added by the resource
configuration. This can be used to help mount a set of routes at a different location
than the included callable's author intended while still maintaining the same resource names.

For example:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/scope.rs:scope}}
```

In the above example, the *show_users* route will have an effective route pattern of
*/users/show* instead of */show* because the application's scope will be prepended
to the pattern. The route will then only match if the URL path is */users/show*,
and when the `HttpRequest.url_for()` function is called with the route name show_users,
it will generate a URL with that same path.

# Custom route guard

You can think of a guard as a simple function that accepts a *request* object reference
and returns *true* or *false*. Formally, a guard is any object that implements the
[`Guard`][guardtrait] trait. Actix provides several predicates, you can check
[functions section][guardfuncs] of API docs.

Here is a simple guard that check that a request contains a specific *header*:

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/guard.rs:guard}}
```

In this example, *index* handler will be called only if request contains *CONTENT-TYPE* header.

Guards can not access or modify the request object, but it is possible to store extra
information in [request extensions][requestextensions].

## Modifying guard values

You can invert the meaning of any predicate value by wrapping it in a `Not` predicate.
For example, if you want to return "METHOD NOT ALLOWED" response for all methods
except "GET":

```rust,edition2018,no_run,noplaypen
{{#include ../examples/url-dispatch/src/guard2.rs:guard2}}
```

The `Any` guard accepts a list of guards and matches if any of the supplied
guards match. i.e:

```rust
guard::Any(guard::Get()).or(guard::Post())
```

The `All` guard accepts a list of guard and matches if all of the supplied
guards match. i.e:

```rust
guard::All(guard::Get()).and(guard::Header("content-type", "plain/text"))
```

# Changing the default Not Found response

If the path pattern can not be found in the routing table or a resource can not find matching
route, the default resource is used. The default response is *NOT FOUND*.
It is possible to override the *NOT FOUND* response with `App::default_service()`.
This method accepts a *configuration function* same as normal resource configuration
with `App::service()` method.

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
