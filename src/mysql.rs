use anyhow::Result;
use itertools::Itertools;
use sqlx::{
    Connection, Executor, MySqlConnection, MySqlPool,
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
pub struct TestMySql {
    pub server_url: String,
    pub dbname: String,
}

/// Builder for creating a TestMySql instance with custom configuration.
pub struct TestMySqlBuilder<S>
where
    S: MigrationSource<'static> + Send + Sync + 'static,
{
    database_url: String,
    migrations: S,
    seeds_path: Option<PathBuf>,
}

impl<S> TestMySqlBuilder<S>
where
    S: MigrationSource<'static> + Send + Sync + 'static,
{
    /// Create a new TestMySqlBuilder with the given database URL and migrations.
    pub fn new(database_url: String, migrations: S) -> Self {
        Self {
            database_url,
            migrations,
            seeds_path: None,
        }
    }

    /// Add a path to a directory containing seed SQL files.
    ///
    /// Seed files should be named with the pattern `<timestamp>_<description>.sql`
    /// (e.g., `20240101120000_initial_data.sql`). They will be executed in
    /// timestamp order after migrations are complete.
    ///
    /// # Example
    /// ```no_run
    /// use sqlx_db_tester::TestMySqlBuilder;
    /// use std::path::Path;
    ///
    /// let tdb = TestMySqlBuilder::new(
    ///     "mysql://root:password@127.0.0.1:3307".to_string(),
    ///     Path::new("./fixtures/mysql_migrations")
    /// )
    /// .with_seeds(Path::new("./fixtures/seeds"))
    /// .build();
    /// ```
    pub fn with_seeds<P: AsRef<Path>>(mut self, seeds_path: P) -> Self {
        self.seeds_path = Some(seeds_path.as_ref().to_path_buf());
        self
    }

    /// Build and initialize the test database with the configured settings.
    pub fn build(self) -> TestMySql {
        TestMySql::new_with_config(self.database_url, self.migrations, self.seeds_path)
    }
}

impl TestMySql {
    pub fn new<S>(database_url: String, migrations: S) -> Self
    where
        S: MigrationSource<'static> + Send + Sync + 'static,
    {
        Self::new_with_config(database_url, migrations, None)
    }

    fn new_with_config<S>(database_url: String, migrations: S, seeds_path: Option<PathBuf>) -> Self
    where
        S: MigrationSource<'static> + Send + Sync + 'static,
    {
        let simple = Uuid::new_v4().simple();
        let (server_url, dbname) = parse_mysql_url(&database_url);
        let dbname = match dbname {
            Some(db_name) => format!("{db_name}_test_{simple}"),
            None => format!("test_{simple}"),
        };
        let dbname_cloned = dbname.clone();
        let server_url_cloned = server_url.clone();

        let tdb = Self { server_url, dbname };

        let url = tdb.url();

        // create database dbname
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                // use server url to create database
                // For MySQL, we always connect to the mysql system database to create a new database
                let create_db_url = format!("{server_url_cloned}/mysql");
                let mut conn = MySqlConnection::connect(&create_db_url)
                    .await
                    .unwrap_or_else(|_| panic!("Error while connecting to {create_db_url}"));
                conn.execute(format!(r#"CREATE DATABASE `{dbname_cloned}`"#).as_str())
                    .await
                    .unwrap();

                // now connect to test database for migration
                let mut conn = MySqlConnection::connect(&url)
                    .await
                    .unwrap_or_else(|_| panic!("Error while connecting to {}", &url));
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

    pub async fn get_pool(&self) -> MySqlPool {
        let url = self.url();
        MySqlPool::connect(&url)
            .await
            .unwrap_or_else(|_| panic!("Error while connecting to {url}"))
    }

    pub async fn load_csv(&self, table: &str, _fields: &[&str], filename: &Path) -> Result<()> {
        // For MySQL, we read the file and use load_csv_data since LOAD DATA LOCAL INFILE
        // requires complex setup and the file needs to be accessible from the MySQL process
        let csv_content = std::fs::read_to_string(filename)?;
        self.load_csv_data(table, &csv_content).await
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

impl Drop for TestMySql {
    fn drop(&mut self) {
        let server_url = &self.server_url;
        let database_url = format!("{server_url}/mysql");
        let dbname = self.dbname.clone();
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                let mut conn = MySqlConnection::connect(&database_url)
                    .await
                    .unwrap_or_else(|_| panic!("Error while connecting to {database_url}"));
                conn.execute(format!(r#"DROP DATABASE `{dbname}`"#).as_str())
                    .await
                    .expect("Error while querying the drop database");
            });
        })
        .join()
        .expect("failed to drop database");
    }
}

impl Default for TestMySql {
    fn default() -> Self {
        Self::new(
            "mysql://root:password@127.0.0.1:3307".to_string(),
            Path::new("./fixtures/mysql_migrations"),
        )
    }
}

/// Discovers and runs seed SQL files from a directory.
///
/// Seed files should follow the naming pattern: `<timestamp>_<description>.sql`
/// They will be executed in timestamp order.
async fn run_seeds(conn: &mut MySqlConnection, seeds_dir: &Path) -> Result<()> {
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

fn parse_mysql_url(url: &str) -> (String, Option<String>) {
    let url_without_protocol = url.trim_start_matches("mysql://");

    let parts: Vec<&str> = url_without_protocol.split('/').collect();
    let server_url = format!("mysql://{}", parts[0]);

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

    use crate::mysql::TestMySql;
    use anyhow::Result;

    #[tokio::test]
    #[ignore = "requires MySQL server running on 127.0.0.1:3307"]
    async fn test_mysql_should_create_and_drop() {
        let tdb = TestMySql::default();
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
    #[ignore = "requires MySQL server running on 127.0.0.1:3307"]
    async fn test_mysql_should_load_csv() -> Result<()> {
        let filename = env::current_dir()?.join("fixtures/todos.csv");
        let tdb = TestMySql::default();
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
    #[ignore = "requires MySQL server running on 127.0.0.1:3307"]
    async fn test_mysql_should_load_csv_data() -> Result<()> {
        let csv = include_str!("../fixtures/todos.csv");
        let tdb = TestMySql::default();
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

    #[test]
    fn test_with_dbname() {
        let url = "mysql://testuser:1@localhost/testdb";
        let (server_url, dbname) = parse_mysql_url(url);
        assert_eq!(server_url, "mysql://testuser:1@localhost");
        assert_eq!(dbname, Some("testdb".to_string()));
    }

    #[test]
    fn test_without_dbname() {
        let url = "mysql://testuser:1@localhost";
        let (server_url, dbname) = parse_mysql_url(url);
        assert_eq!(server_url, "mysql://testuser:1@localhost");
        assert_eq!(dbname, None);
    }

    #[tokio::test]
    #[ignore = "requires MySQL server running on 127.0.0.1:3307"]
    async fn test_mysql_with_seeds() {
        use crate::TestMySqlBuilder;

        let tdb = TestMySqlBuilder::new(
            "mysql://root:password@127.0.0.1:3307".to_string(),
            Path::new("./fixtures/mysql_migrations"),
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
