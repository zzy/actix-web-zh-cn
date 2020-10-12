# actix-web 中文文档

[Build Status travis]: https://api.travis-ci.com/zzy/actix-web-zh-cn.svg?branch=master
[travis]: https://travis-ci.com/zzy/actix-web-zh-cn

actix-web 是 Rust 生态中的 web 框架，具有类型安全、功能丰富、扩展性强，以及速度极快的诸多优点。

**感谢 actix 团队的无私奉献！**

本仓库是对 actix-web 的中文翻译文档，同步 actix 团队仓库和官网。

## 在线阅读

在线阅读地址：[**《actix-web 中文文档》** - https://actix-web.budshome.com](https://actix-web.budshome.com)。

## 离线阅读

如果你喜欢本地阅读方式，可以使用 mdBook（[中文文档](https://mdbook.budshome.com)） 进行书籍构建：

> 构建时需要安装一些 crate，中国大陆推荐[更换默认的 Cargo 源为国内镜像源](https://cargo.budshome.com/reference/source-replacement.html)。

```bash
$ git clone https://github.com/zzy/actix-web-zh-cn
$ cd actix-web-zh-cn
$ cargo install mdbook # 指定版本使用参数：--vers "0.3.5"
$ mdbook serve --open # 或者 mdbook build
```

也可以直接用你喜欢的浏览器从 `book` 子目录打开 `index.html` 文件。

```bash
$ xdg-open ./book/index.html # linux
$ start .\book\index.html    # windows
$ open ./book/index.html     # mac
```

## 贡献

《actix-web 中文文档》的目的是让 Rust 程序员新手能够更容易地参与到 actix-web 社区中，因此它需要——并欢迎——你做出自己力所能及的贡献。

祝你学习愉快，欢迎提交问题，欢迎发送 PR。
