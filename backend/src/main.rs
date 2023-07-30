use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Json, Router,
};
use axum::response::Redirect;
use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    // Create database with connection and share it using with_state below
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    // Create router for server
    let app = Router::new()
        .route("/", get(list_project))
        .route("/createProject", post(create_project))
        .route("/readProject/:id", get(read_project))
        .route("/updateProject", get(update_project))
        .route("/deleteProject/:id", post(delete_project))
        .route("/todos/:id", get(list_todo))
        .route("/createTodo", post(create_todo))
        .route("/readTodo/:id", get(read_todo))
        .route("/updateTodo", get(update_todo))
        .route("/deleteTodo/:id", post(delete_todo))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    // Start server!
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Starting server on http://{address}");
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

#[derive(Deserialize)]
struct NewProject {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Project {
    id: i64,
    description: String,
}

#[derive(Deserialize)]
struct NewTodo {
    project_id: i64,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    project_id: i64,
    description: String,
    done: bool,
}

async fn list_project(State(pool): State<SqlitePool>) -> Result<Json<Vec<Project>>> {
    let projects = sqlx::query_as!(Project, "SELECT id, description FROM project ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(projects))
}

async fn create_project(State(pool): State<SqlitePool>, Form(project): Form<NewProject>) -> Result<Json<bool>> {
    sqlx::query!(
        "INSERT INTO project (description) VALUES (?)",
        project.description,
    )
        .execute(&pool)
        .await?;
    Ok(Json(true))
}

async fn read_project(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Project>> {
    let project = sqlx::query_as!(
        Project,
        "SELECT id, description FROM project WHERE id = ?",
        id
    )
        .fetch_one(&pool)
        .await?;
    Ok(Json(project))
}

async fn update_project(State(pool): State<SqlitePool>, Form(project): Form<Project>) -> Result<Json<bool>> {
    sqlx::query!(
        "UPDATE project SET description = ? WHERE id = ?",
        project.description,
        project.id
    ).execute(&pool).await?;
    Ok(Json(true))
}

async fn delete_project(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<bool>> {
    sqlx::query!("DELETE FROM project WHERE id = ?", id)
        .execute(&pool)
        .await?;
    Ok(Json(true))
}

async fn list_todo(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Vec<Todo>>> {
    let todos = sqlx::query_as!(Todo, "SELECT id, description, project_id, done FROM todo WHERE id = ?",
        id
    )
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create_todo(State(pool): State<SqlitePool>, Form(todo): Form<NewTodo>) -> Result<Json<bool>> {
    sqlx::query!(
        "INSERT INTO todo (project_id, description) VALUES (?, ?)",
        todo.project_id,
        todo.description
    )
        .execute(&pool)
        .await?;
    Ok(Json(true))
}

async fn read_todo(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Todo>> {
    let todo = sqlx::query_as!(
        Todo,
        "SELECT id, description, project_id, done FROM todo WHERE id = ?",
        id
    )
        .fetch_one(&pool)
        .await?;
    Ok(Json(todo))
}

async fn update_todo(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<Json<bool>> {
    sqlx::query!(
        "UPDATE todo SET description = ?, done = ? WHERE id = ?",
        todo.description,
        todo.done,
        todo.id
    ).execute(&pool).await?;
    Ok(Json(true))
}

async fn delete_todo(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<bool>> {
    sqlx::query!("DELETE FROM todo WHERE id = ?", id)
        .execute(&pool)
        .await?;
    Ok(Json(true))
}