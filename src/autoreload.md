# 自动重载开发服务器

> [autoreload.md](https://github.com/actix/actix-website/blob/master/content/docs/autoreload.md)
> <br />
> commit - fa45d6e0376cf6299db25bf1e4d6a48a53516a6e - 2020.11.25

开发过程中，在代码修改时让 cargo 自动重新编译是非常方便的。这可以很容易地通过 [`cargo-watch`] 实现。

```sh
cargo watch -x 'run --bin app'
```

## 历史笔记

旧版本中，我们建议使用 systemfd 和 listenfd 的组合，但这有很多问题。尤其是在更广泛的开发工作流中，很难正确集成。我们认为，[`cargo-watch`] 足够实现自动重载目标。

[`cargo-watch`]: https://github.com/passcod/cargo-watch
