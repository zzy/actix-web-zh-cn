# 自动重载开发服务器

> [autoreload.md](https://github.com/actix/actix-website/blob/master/content/docs/autoreload.md)
> <br />
> commit - fa45d6e0376cf6299db25bf1e4d6a48a53516a6e - 2020.11.25

在开发过程中，让cargo在更改时自动重新编译代码非常方便。这可以很容易地完成使用货物手表。



During development it can be very handy to have cargo automatically recompile the code on changes.
This can be accomplished very easily by using [`cargo-watch`].

```sh
cargo watch -x 'run --bin app'
```

## Historical Note

An old version of this page recommended using a combination of systemfd and listenfd, but this has
many gotchas and was difficult to integrate properly, especially when part of a broader development
workflow. We consider [`cargo-watch`] to be sufficient for auto-reloading purposes.

[`cargo-watch`]: https://github.com/passcod/cargo-watch
