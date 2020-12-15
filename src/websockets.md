# WebSockets

> [websockets.md](https://github.com/actix/actix-website/blob/master/content/docs/websockets.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

actix-web 通过 `actix-web-actors` crate 支持 WebSockets。可以将请求的`有效负载`转换为具有 [*web::Payload*][payload] 结构体的枚举消息流 [*ws::Message*][message]，然后使用流组合器来处理实际消息，但是使用 http actor 处理 websocket 通信更简单。

以下是一个简单的 websocket 回音（echo）服务示例：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/websockets/src/main.rs:websockets}}
```

> [实例文件夹][examples]中提供了一个简单的 websocket 回音（echo）服务示例。

> [websocket-chat 文件夹][chat]中提供了一个聊天服务示例，它能够通过 websocket 或 TCP 连接进行聊天。

[message]: https://docs.rs/actix-web-actors/2/actix_web_actors/ws/enum.Message.html
[payload]: https://docs.rs/actix-web/3/actix_web/web/struct.Payload.html
[examples]: https://github.com/actix/examples/tree/master/websocket/
[chat]: https://github.com/actix/examples/tree/master/websocket-chat/
