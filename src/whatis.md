# actix crate 属于 Rust 生态 

> [whatis.md](https://github.com/actix/actix-website/blob/master/content/docs/whatis.md)
> <br />
> commit - 4d8d53cea59bca095ca5c02ef81f0b1791736855 - 2020.09.12

actix 包含诸多功能和特性。actix 系统基于强大的 actor 模型构建，在 actix 系统的基础上又构建了 `actix-web` 系统。一般情况下，你都是在 `actix-web` 系统上进行工作，`actix-web` 提供了一个功能强大、速度极快的 web 开发框架。

我们称 `actix-web` 是小巧而实用的框架，总而言之，`actix-web` 是有些变化的微框架（micro-framework）。如果你已经是一个 Rust 程序员，你可能会很快找到感觉。即使你暂不熟悉 Rust 编程语言，你也会发现 `actix-web` 很容易上手。

使用 `actix-web` 开发的应用程序，将有一个 HTTP 服务器包含在可执行文件中。你可以将其放在另一个 HTTP 服务器（如 nginx）之后，或直接提供服务。即使没有其它 HTTP 服务器，`actix-web` 也足够强大：可以提供对 HTTP/1、HTTP/2，以及 TLS（HTTPS）的支持。这使得它对于构建可供分发的小型服务非常有用。

请注意：`actix-web` 运行在 Rust 1.42 或更高的稳定（stable）版本之上。

<!-- TODO -->
<!-- which is built upon the fantastic [Tokio][tokio] asynchronous I/O system -->

<!-- LINKS -->

[tokio]: https://tokio.rs
