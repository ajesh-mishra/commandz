use axum::{
    Json,
    Router, 
    routing::{get, post}
};
use dotenv_codegen::dotenv;
use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgPoolOptions};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_handler))
        .route("/", post(post_handler))
}

#[derive(Debug, Serialize, Deserialize)]
struct Command {
    id: i32,
    command_name: String,
    description: String,
    command_type: String,
	created_on: chrono::NaiveDateTime
}

async fn get_handler() -> Json<Vec<Command>> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await
        .expect("Unable to connect to Postgres");

    let commands: Vec<Command> = sqlx::query_as!(Command, "SELECT * FROM commands")
        .fetch_all(&db)
        .await
        .expect("Unable to query commands table");

    println!("Command: {:#?}", &commands);

    Json(commands)
}

async fn post_handler(Json(payload): Json<Command>) -> Json<Command> {
    println!("\nIn the post function!");
    println!("\nPayload: {:?}\n", &payload);

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await
        .expect("Unable to connect to Postgres");

    // let response = 
    sqlx::query_as!(
            Command, 
            "INSERT INTO commands (command_name, description, command_type, created_on) VALUES ($1, $2, $3, $4)",
            payload.command_name, payload.description, payload.command_type, payload.created_on
        )
        .fetch_all(&db)
        .await
        .expect("Unable to query commands table");

    // println!("Response: {response:?}");

    Json(payload)
}
