use sqlx_db_tester::TestMySql;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a test database with migrations
    let tdb = TestMySql::new(
        "mysql://root:password@127.0.0.1:3307".to_string(),
        Path::new("./fixtures/mysql_migrations"),
    );

    println!("Created test database: {}", tdb.dbname);
    println!("Database URL: {}", tdb.url());

    // Get a connection pool
    let pool = tdb.get_pool().await;

    // Insert a test record
    sqlx::query("INSERT INTO todos (title) VALUES (?)")
        .bind("Test MySQL Todo")
        .execute(&pool)
        .await?;

    // Query the record back
    let (id, title): (i32, String) = sqlx::query_as("SELECT id, title FROM todos WHERE title = ?")
        .bind("Test MySQL Todo")
        .fetch_one(&pool)
        .await?;

    println!("Retrieved todo: id={id}, title={title}");

    // Test CSV loading
    let csv_data = "title\nLoaded from CSV\nAnother CSV entry";
    tdb.load_csv_data("todos", csv_data).await?;

    // Count all todos
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM todos")
        .fetch_one(&pool)
        .await?;

    println!("Total todos in database: {}", count.0);

    // The database will be automatically dropped when tdb goes out of scope
    println!("Test completed successfully! Database will be cleaned up automatically.");

    Ok(())
}
