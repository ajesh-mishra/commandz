// #[macro_use]
// extern crate dotenv_codegen;

// use dotenv::dotenv;

use axum_backend::serve;

// use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// struct Command {
//     id: i32,
//     command_name: String,
//     description: String,
//     command_type: String,
//     created_on: chrono::NaiveDateTime,
// }

#[tokio::main]
async fn main() {
    // dotenv().ok();
    // dotenv::dotenv().expect("Unable to load environment variables from .env file");

    // let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    // let db = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&db_url)
    //     .await
    //     .expect("Unable to connect to Postgres");

    serve().await;
}

// async fn get_commands_handler(db: &Pool<Postgres>) -> Result<String, Error> {
//     let commands: Vec<Command> = sqlx::query_as!(Command, "SELECT * FROM commands")
//         .fetch_all(db)
//         .await
//         .expect("Unable to query commands table");

//     println!("{commands:#?}");

//     // Json(commands)
//     serde_json::to_string(&commands)
// }
