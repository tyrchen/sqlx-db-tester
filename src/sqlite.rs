use sqlx::{
    migrate::{MigrationSource, Migrator},
    Connection, SqliteConnection, SqlitePool,
};
use std::{path::Path, thread};
use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct TestSqlite;

impl TestSqlite {
    pub fn new<S>(migrations: S) -> Self
    where
        S: MigrationSource<'static> + Send + Sync + 'static,
    {
        let tdb = Self {};
        let url = tdb.url();

        // create database dbname
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                // now connect to test database for migration
                let mut conn = SqliteConnection::connect(&url).await.unwrap();
                let m = Migrator::new(migrations).await.unwrap();
                m.run(&mut conn).await.unwrap();
            });
        })
        .join()
        .expect("failed to create database");

        tdb
    }

    pub fn url(&self) -> String {
        "sqlite::memory:".to_owned()
    }

    pub async fn get_pool(&self) -> SqlitePool {
        SqlitePool::connect(&self.url()).await.unwrap()
    }
}

impl Default for TestSqlite {
    fn default() -> Self {
        Self::new(Path::new("./migrations"))
    }
}

#[cfg(test)]
mod tests {
    use crate::TestSqlite;

    #[tokio::test]
    async fn test_sqlite_should_create_and_drop() {
        let tdb = TestSqlite::default();
        let pool = tdb.get_pool().await;
        println!("!!!");
        // insert todo
        sqlx::query("INSERT INTO todos (title) VALUES ('test')")
            .execute(&pool)
            .await
            .unwrap();
        // get todo
        let (id, title) = sqlx::query_as::<_, (i32, String)>("SELECT id, title FROM todos")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(id, 1);
        assert_eq!(title, "test");
    }
}
