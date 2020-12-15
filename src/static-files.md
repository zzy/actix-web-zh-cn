# 静态文件

> [static-files.md](https://github.com/actix/actix-website/blob/master/content/docs/static-files.md)
> <br />
> commit - 529bebcd66999fdd46e759712394c32e3bc0ed3a - 2020.02.11

## 单个文件

可以使用自定义路径模式和 `NamedFile` 提供服务所需静态文件，为了匹配路径尾部，我们可以使用正则表达式 `[.*]`。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/static-files/src/main.rs:individual-file}}
```

## 目录

为了提供来自特定目录和其子目录的文件，可以使用 `Files`。`Files` 必须通过 `App::service()` 方法注册，否则它将无法为子路径提供服务。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/static-files/src/directory.rs:directory}}
```

默认情况下，子目录的文件列表被禁用。尝试加载目录列表将返回响应 *404 Not Found*。要启用文件列表，请使用 [*Files::show_files_listing()*][showfileslisting] 方法。

可以重定向到特定的索引文件，而不是显示目录的文件列表。重定向可使用 [*Files::index_file()*][indexfile] 方法配置。

## 配置

`NamedFiles` 可以指定服务文件的各种选项：

- `set_content_disposition` - 此函数用于将文件的 mime 值映射到相应的 `Content-Disposition` 类型。
- `use_etag` - 指定是否应计算 `ETag` 并将其包含在消息标头中。
- `use_last_modified` - 指定是否应使用文件修改的时间戳，并将其添加到消息标头 `Last-Modified`。

以上所有设定方法都是可选的，并提供了最佳的默认值，但是你可以自定义其中任何一个。

```rust,edition2018,no_run,noplaypen
{{#include ../examples/static-files/src/configuration.rs:config-one}}
```

该配置也可以应用于目录服务：

```rust,edition2018,no_run,noplaypen
{{#include ../examples/static-files/src/configuration_two.rs:config-two}}
```

[showfileslisting]: https://docs.rs/actix-files/0.2/actix_files/struct.Files.html
[indexfile]: https://docs.rs/actix-files/0.2/actix_files/struct.Files.html#method.index_file
