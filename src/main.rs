use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

// #[async_std::main]
#[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres").await?; // 5432

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
    .bind(150_i64)
    .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    // Make a simple query to real table and return value from its column:
    // CREATE TABLE public.test (
    //     "name" varchar NULL,
    //     age int4 NULL,
    //     id int4 NOT NULL GENERATED ALWAYS AS IDENTITY
    // ); 
    let row: (String,) = sqlx::query_as("SELECT name from public.test")
        .fetch_one(&pool).await?; 
    println!("{:?}", &row);
 
    Ok(())
}