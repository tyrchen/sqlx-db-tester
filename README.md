![](https://github.com/tyrchen/sqlx-db-tester/workflows/build/badge.svg)

# sqlx-db-tester

This a tool to test sqlx with postgres. It only supports tokio runtime at this moment.

## How to use it

You should first create a `TestDb` data structure in your tests. It will automatically create a database and a connection pool for you. You could then get the connection string or connection pool from it to use in your own code. When `TestDb` gets dropped, it will automatically drop the database.

```rust
#[tokio::test]
fn some_awesom_test() {
    let tdb = TestDb::new("localhost", 5432, "postgres", "postgres", "./migrations");
    let pool = tdb.get_pool().await;
    // do something with the pool

    // when tdb gets dropped, the database will be dropped
}
```

Have fun with this crate!

## License

This project is distributed under the terms of MIT.

See [LICENSE](LICENSE.md) for details.

Copyright 2022 Tyr Chen
