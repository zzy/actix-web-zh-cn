# 测试

> [testing.md](https://github.com/actix/actix-website/blob/master/content/docs/testing.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

每个应用程序都应当经过良好的测试，actix-web 提供了用于执行单元和集成测试的工具。

## 单元测试

对于单元测试，actix-web 提供了一个请求 builder 类型。[*TestRequest*][testrequest] 实现了一个类似于 builder 的模式。你可以使用 `to_http_request()` 生成 `HttpRequest` 实例，并用它调用 handler 函数。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/testing/src/main.rs:unit-tests}}
```

## 集成测试

测试应用程序有多种方式。actix-web 可用于在真实 HTTP 服务器上运行具有指定处理程序的应用。

可以使用 `TestRequest::get()`、`TestRequest::post()`，以及其它方法，向测试服务器发送请求。

要创建用于测试的 `Service`，使用 `test::init_service` 方法，它可以接受常规的 `App` 构建器。

> 查阅 [API 文档][actixdocs]以了解更多信息。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/testing/src/integration_one.rs:integration-one}}
```

即使你需要更复杂的应用程序配置，测试也是与创建普通应用程序非常相似的。例如，你可能需要初始化应用程序状态，使用 `data` 方法创建 `App` 实例并附加状态，就像在普通应用程序中所做的一样。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/testing/src/integration_two.rs:integration-two}}
```

## 流响应测试

如果你需要测试流生成，只需调用 `take_body()` 并将结果 [*ResponseBody*][responsebody] 转换为 future，然后执行它。例如，下面示例测试[*服务器发送事件*][serversentevents]。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/testing/src/stream_response.rs:stream-response}}
```

[serversentevents]: https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events
[responsebody]: https://docs.rs/actix-web/3/actix_web/body/enum.ResponseBody.html
[actixdocs]: https://docs.rs/actix-web/3/actix_web/test/index.html
[testrequest]: https://docs.rs/actix-web/3/actix_web/test/struct.TestRequest.html
