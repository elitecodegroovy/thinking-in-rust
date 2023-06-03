#![warn(clippy::all)]

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use chrono::prelude::*;
use serde_json::json;
// use sqlx::mysql::MySqlPoolOptions;
// etc.


fn do_direct_protocol() {
    let key = "YELLOW SUBMARINE, BLACK WIZARDRY".as_bytes();
    let mut key_mut = Vec::from(key);
    let message = "This is a signed non-JSON message.";
    let footer = "key-id:gandalf0";

    // Version 2
    let v2_token =
        paseto::v2::local::local_paseto(&message, None, &mut key_mut).expect("Failed to encrypt V2 Token sans footer.");
    println!("{:?}", v2_token);
    let decrypted_v2_token = paseto::v2::local::decrypt_paseto(&v2_token, None, &mut key_mut)
        .expect("Failed to decrypt V2 Token sans footer.");
    println!("{:?}", decrypted_v2_token);
    let v2_footer_token =
        paseto::v2::local::local_paseto(&message, Some(&footer), &mut key_mut).expect("Failed to encrypt V2 Token.");
    println!("{:?}", v2_footer_token);
    let decrypted_v2_footer = paseto::v2::local::decrypt_paseto(&v2_footer_token, Some(&footer), &mut key_mut)
        .expect("Failed to decrypt V2 Token.");
    println!("{:?}", decrypted_v2_footer);
}

// #[async_std::main]
#[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {

    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(2000)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64, ) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    do_direct_protocol();

    Ok(())
}