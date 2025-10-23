use anyhow::Result;
use itertools::Itertools;
use sqlx::{
    Connection, Executor, PgConnection, PgPool,
    migrate::{MigrationSource, Migrator},
};
use std::{
    fs,
    path::{Path, PathBuf},
    thread,
};
use tokio::runtime::Runtime;
use uuid::Uuid;

#[derive(Debug)]
pub struct TestPg {
    pub server_url: String,
    pub dbname: String,
    #[allow(dead_code)]
    extensions: Vec<String>,
}

/// Builder for creating a TestPg instance with custom configuration.
pub struct TestPgBuilder<S>
where
    S: MigrationSource<'static> + Send + Sync + 'static,
{
    database_url: String,
    migrations: S,
    extensions: Vec<String>,
    seeds_path: Option<PathBuf>,
}

impl<S> TestPgBuilder<S>
where
    S: MigrationSource<'static> + Send + Sync + 'static,
{
    /// Create a new TestPgBuilder with the given database URL and migrations.
    pub fn new(database_url: String, migrations: S) -> Self {
        Self {
            database_url,
            migrations,
            extensions: vec![],
            seeds_path: None,
        }
    }

    /// Add a list of PostgreSQL extensions to be installed before running migrations.
    ///
    /// # Example
    /// ```no_run
    /// use sqlx_db_tester::TestPgBuilder;
    /// use std::path::Path;
    ///
    /// let tdb = TestPgBuilder::new(
    ///     "postgres://postgres:postgres@localhost:5432".to_string(),
    ///     Path::new("./fixtures/migrations")
    /// )
    /// .with_extensions(vec!["uuid-ossp".to_string(), "postgis".to_string()])
    /// .build();
    /// ```
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }

    /// Add a path to a directory containing seed SQL files.
    ///
    /// Seed files should be named with the pattern `<timestamp>_<description>.sql`
    /// (e.g., `20240101120000_initial_data.sql`). They will be executed in
    /// timestamp order after migrations are complete.
    ///
    /// # Example
    /// ```no_run
    /// use sqlx_db_tester::TestPgBuilder;
    /// use std::path::Path;
    ///
    /// let tdb = TestPgBuilder::new(
    ///     "postgres://postgres:postgres@localhost:5432".to_string(),
    ///     Path::new("./fixtures/migrations")
    /// )
    /// .with_seeds(Path::new("./fixtures/seeds"))
    /// .build();
    /// ```
    pub fn with_seeds<P: AsRef<Path>>(mut self, seeds_path: P) -> Self {
        self.seeds_path = Some(seeds_path.as_ref().to_path_buf());
        self
    }

    /// Build and initialize the test database with the configured settings.
    pub fn build(self) -> TestPg {
        TestPg::new_with_config(
            self.database_url,
            self.migrations,
            self.extensions,
            self.seeds_path,
        )
    }
}

impl TestPg {
    pub fn new<S>(database_url: String, migrations: S) -> Self
    where
        S: MigrationSource<'static> + Send + Sync + 'static,
    {
        Self::new_with_config(database_url, migrations, vec![], None)
    }

    #[allow(dead_code)]
    fn new_with_extensions<S>(database_url: String, migrations: S, extensions: Vec<String>) -> Self
    where
        S: MigrationSource<'static> + Send + Sync + 'static,
    {
        Self::new_with_config(database_url, migrations, extensions, None)
    }

    fn new_with_config<S>(
        database_url: String,
        migrations: S,
        extensions: Vec<String>,
        seeds_path: Option<PathBuf>,
    ) -> Self
    where
        S: MigrationSource<'static> + Send + Sync + 'static,
    {
        let simple = Uuid::new_v4().simple();
        let (server_url, dbname) = parse_postgres_url(&database_url);
        let dbname = match dbname {
            Some(db_name) => format!("{db_name}_test_{simple}"),
            None => format!("test_{simple}"),
        };
        let dbname_cloned = dbname.clone();
        let extensions_cloned = extensions.clone();

        let tdb = Self {
            server_url,
            dbname,
            extensions,
        };

        let url = tdb.url();

        // create database dbname
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                // use server url to create database
                let mut conn = PgConnection::connect(&database_url)
                    .await
                    .unwrap_or_else(|_| panic!("Error while connecting to {database_url}"));
                conn.execute(format!(r#"CREATE DATABASE "{dbname_cloned}""#).as_str())
                    .await
                    .unwrap();

                // now connect to test database for migration
                let mut conn = PgConnection::connect(&url)
                    .await
                    .unwrap_or_else(|_| panic!("Error while connecting to {}", &url));

                // create extensions before running migrations
                for ext in &extensions_cloned {
                    conn.execute(format!(r#"CREATE EXTENSION IF NOT EXISTS "{ext}""#).as_str())
                        .await
                        .unwrap_or_else(|_| panic!("Error while creating extension {ext}"));
                }

                let m = Migrator::new(migrations).await.unwrap();
                m.run(&mut conn).await.unwrap();

                // run seed files if provided
                if let Some(seeds_dir) = seeds_path {
                    run_seeds(&mut conn, &seeds_dir).await.unwrap();
                }
            });
        })
        .join()
        .expect("failed to create database");

        tdb
    }

    pub fn server_url(&self) -> String {
        self.server_url.clone()
    }

    pub fn url(&self) -> String {
        format!("{}/{}", self.server_url, self.dbname)
    }

    pub async fn get_pool(&self) -> PgPool {
        let url = self.url();
        PgPool::connect(&url)
            .await
            .unwrap_or_else(|_| panic!("Error while connecting to {url}"))
    }

    pub async fn load_csv(&self, table: &str, fields: &[&str], filename: &Path) -> Result<()> {
        let pool = self.get_pool().await;
        let path = filename.canonicalize()?;
        let mut conn = pool.acquire().await?;
        let sql = format!(
            "COPY {} ({}) FROM '{}' DELIMITER ',' CSV HEADER;",
            table,
            fields.join(","),
            path.display()
        );
        conn.execute(sql.as_str()).await?;
        // copy csv

        Ok(())
    }

    pub async fn load_csv_data(&self, table: &str, csv: &str) -> Result<()> {
        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.headers()?.iter().join(",");
        let mut tx = self.get_pool().await.begin().await?;
        for result in rdr.records() {
            let record = result?;
            let sql = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table,
                headers,
                record.iter().map(|v| format!("'{v}'")).join(",")
            );
            tx.execute(sql.as_str()).await?;
        }
        tx.commit().await?;
        Ok(())
    }
}

impl Drop for TestPg {
    fn drop(&mut self) {
        let server_url = &self.server_url;
        let database_url = format!("{server_url}/postgres");
        let dbname = self.dbname.clone();
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                    let mut conn = PgConnection::connect(&database_url).await
                    .unwrap_or_else(|_| panic!("Error while connecting to {database_url}"));
                    // terminate existing connections
                    sqlx::query(&format!(r#"SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE pid <> pg_backend_pid() AND datname = '{dbname}'"#))
                    .execute( &mut conn)
                    .await
                    .expect("Terminate all other connections");
                    conn.execute(format!(r#"DROP DATABASE "{dbname}""#).as_str())
                        .await
                        .expect("Error while querying the drop database");
                });
            })
            .join()
            .expect("failed to drop database");
    }
}

impl Default for TestPg {
    fn default() -> Self {
        Self::new(
            "postgres://postgres:postgres@localhost:5432".to_string(),
            Path::new("./fixtures/migrations"),
        )
    }
}

/// Discovers and runs seed SQL files from a directory.
///
/// Seed files should follow the naming pattern: `<timestamp>_<description>.sql`
/// They will be executed in timestamp order.
async fn run_seeds(conn: &mut PgConnection, seeds_dir: &Path) -> Result<()> {
    if !seeds_dir.exists() {
        return Ok(());
    }

    let mut seed_files = Vec::new();

    // read all .sql files from the seeds directory
    for entry in fs::read_dir(seeds_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && path.extension().is_some_and(|ext| ext == "sql")
            && let Some(filename) = path.file_name().and_then(|n| n.to_str())
        {
            // extract timestamp from filename (before first underscore)
            if let Some(timestamp) = filename.split('_').next() {
                seed_files.push((timestamp.to_string(), path));
            }
        }
    }

    // sort by timestamp
    seed_files.sort_by(|a, b| a.0.cmp(&b.0));

    // execute each seed file
    for (_timestamp, path) in seed_files {
        let sql = fs::read_to_string(&path)?;
        conn.execute(sql.as_str()).await?;
    }

    Ok(())
}

fn parse_postgres_url(url: &str) -> (String, Option<String>) {
    let url_without_protocol = url.trim_start_matches("postgres://");

    let parts: Vec<&str> = url_without_protocol.split('/').collect();
    let server_url = format!("postgres://{}", parts[0]);

    let dbname = if parts.len() > 1 && !parts[1].is_empty() {
        Some(parts[1].to_string())
    } else {
        None
    };

    (server_url, dbname)
}
#[cfg(test)]
mod tests {
    use std::env;

    use crate::postgres::TestPg;
    use anyhow::Result;

    #[tokio::test]
    async fn test_postgres_should_create_and_drop() {
        let tdb = TestPg::default();
        let pool = tdb.get_pool().await;
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

    #[tokio::test]
    #[ignore = "github action postgres server can't be used for this test"]
    async fn test_postgres_should_load_csv() -> Result<()> {
        let filename = env::current_dir()?.join("fixtures/todos.csv");
        let tdb = TestPg::default();
        tdb.load_csv("todos", &["title"], &filename).await?;
        let pool = tdb.get_pool().await;
        // get todo
        let (id, title) = sqlx::query_as::<_, (i32, String)>("SELECT id, title FROM todos")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(id, 1);
        assert_eq!(title, "hello world");
        Ok(())
    }

    #[tokio::test]
    async fn test_postgres_should_load_csv_data() -> Result<()> {
        let csv = include_str!("../fixtures/todos.csv");
        let tdb = TestPg::default();
        tdb.load_csv_data("todos", csv).await?;
        let pool = tdb.get_pool().await;
        // get todo
        let (id, title) = sqlx::query_as::<_, (i32, String)>("SELECT id, title FROM todos")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(id, 1);
        assert_eq!(title, "hello world");
        Ok(())
    }
    use super::*;

    #[tokio::test]
    async fn test_postgres_with_extensions() {
        use crate::TestPgBuilder;

        let tdb = TestPgBuilder::new(
            "postgres://postgres:postgres@localhost:5432".to_string(),
            Path::new("./fixtures/migrations"),
        )
        .with_extensions(vec!["uuid-ossp".to_string()])
        .build();

        let pool = tdb.get_pool().await;

        // Verify the extension is installed by trying to use it
        let result = sqlx::query_scalar::<_, String>("SELECT uuid_generate_v4()::text")
            .fetch_one(&pool)
            .await;

        assert!(result.is_ok(), "uuid-ossp extension should be available");
    }

    #[test]
    fn test_with_dbname() {
        let url = "postgres://testuser:1@localhost/pureya";
        let (server_url, dbname) = parse_postgres_url(url);
        assert_eq!(server_url, "postgres://testuser:1@localhost");
        assert_eq!(dbname, Some("pureya".to_string()));
    }

    #[test]
    fn test_without_dbname() {
        let url = "postgres://testuser:1@localhost";
        let (server_url, dbname) = parse_postgres_url(url);
        assert_eq!(server_url, "postgres://testuser:1@localhost");
        assert_eq!(dbname, None);
    }

    #[tokio::test]
    async fn test_postgres_with_seeds() {
        use crate::TestPgBuilder;

        let tdb = TestPgBuilder::new(
            "postgres://postgres:postgres@localhost:5432".to_string(),
            Path::new("./fixtures/migrations"),
        )
        .with_seeds(Path::new("./fixtures/seeds"))
        .build();

        let pool = tdb.get_pool().await;

        // Verify that seed data was loaded in the correct order
        let todos = sqlx::query_as::<_, (i32, String)>("SELECT id, title FROM todos ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(todos.len(), 3);
        assert_eq!(todos[0].1, "First seeded todo");
        assert_eq!(todos[1].1, "Second seeded todo");
        assert_eq!(todos[2].1, "Third seeded todo");
    }
}
