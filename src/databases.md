---
title: Databases
menu: docs_patterns
weight: 1010
---

# Async Options

We have several example projects showing use of async database adapters:

- SQLx: https://github.com/actix/examples/tree/master/sqlx_todo
- Postgres: https://github.com/actix/examples/tree/master/async_pg
- SQLite: https://github.com/actix/examples/tree/master/async_db

# Diesel

The current version of Diesel (v1) does not support asynchronous operations, so it is important to
use the [`web::block`][web-block] function to offload your database operations to the Actix runtime
thread-pool.

You can create action functions that correspond to all the operations your app will perform on the
database.

{{< include-example example="databases" file="main.rs" section="handler" >}}

Now you should set up the database pool using a crate such as `r2d2`, which makes many DB
connections available to your app. This means that multiple handlers can manipulate the DB at the
same time, and still accept new connections. Simply, the pool in your app state. (In this case, it's
beneficial not to use a state wrapper struct because the pool handles shared access for you.)

{{< include-example example="databases" file="main.rs" section="main" >}}

Now, in a request handler, use the `Data<T>` extractor to get the pool from app state and get a
connection from it. This provides an owned database connection that can be passed into a
[`web::block`][web-block] closure. Then just call the action function with the necessary arguments
and `.await` the result.

This example also maps the error to an `HttpResponse` before using the `?` operator but this is not
necessary if your return error type implements [`ResponseError`][response-error].

{{< include-example example="databases" file="main.rs" section="index" >}}

That's it! See the full example here: https://github.com/actix/examples/tree/master/diesel

[web-block]: https://docs.rs/actix-web/3/actix_web/web/fn.block.html
[response-error]: https://docs.rs/actix-web/3/actix_web/trait.ResponseError.html