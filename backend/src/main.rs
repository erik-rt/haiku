use std::net::SocketAddr;

use axum::response::Redirect;
use axum::{
    extract::{Path, State},
    routing::{get, post, patch},
    Form, Json, Router, Server,
};
use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;

    let pool = SqlitePool::connect(&url).await?;

    // Create the router for the server
    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/read/:id", get(read))
        .route("/update/:id", patch(update))
        .route("/delete/:id", post(delete))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Starting server on http://{address}");
    let server = Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(server)
}

#[derive(Deserialize)]
struct NewHaiku {
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Haiku {
    id: i64,
    content: String,
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Haiku>>> {
    // List haikus
    let haikus = sqlx::query_as!(Haiku, "SELECT id, content FROM haikus ORDER BY id")
        .fetch_all(&pool)
        .await?;

    Ok(Json(haikus))
}

async fn create(State(pool): State<SqlitePool>, Form(haiku): Form<NewHaiku>) -> Result<Redirect> {
    // Create new haiku
    sqlx::query!("INSERT INTO haikus (content) VALUES (?)", haiku.content)
        .execute(&pool)
        .await?;

    // Ok(format!("Successfully inserted haiku"))
    Ok(Redirect::to("http://localhost:5173"))
}

async fn read(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Haiku>> {
    // Read haiku
    let haiku = sqlx::query_as!(Haiku, "SELECT id, content FROM haikus WHERE id = ?", id)
        .fetch_one(&pool)
        .await?;
    Ok(Json(haiku))
}

async fn update(State(pool): State<SqlitePool>, Json(haiku): Json<Haiku>) -> Result<Redirect> {
    // Update haiku
    sqlx::query!(
        "UPDATE haikus SET content = ? WHERE id = ?",
        haiku.content,
        haiku.id
    )
    .execute(&pool)
    .await?;

    Ok(Redirect::to("http://localhost:5173"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Redirect> {
    // Delete haiku
    sqlx::query!("DELETE FROM haikus WHERE id = ?", id)
        .execute(&pool)
        .await?;

    Ok(Redirect::to("http://localhost:5173"))
}
