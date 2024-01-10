use anyhow::Result;
use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router, http::StatusCode,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{query, MySqlPool};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Database connection
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@localhost/main")
        .await?;
    // build our application with a route
    let app = Router::new()
        .merge(routes())
        .layer(middleware::map_response(main_response_mapper));
    // run it
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    // Queries

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("{:<12} - main_response_mapper", "RES_MAPPER");
    res
}

fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/json", get(json))
        .route("/api/hello", get(hello))
        .route("/api/hello2/:name", get(hello2))
        .route("/api/user_a", post(add_user))
}

struct AppError(anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}


#[derive(serde::Serialize, serde::Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    name: Option<String>,
    email: String,
    pasword: String,
}

#[derive(Debug, Deserialize)]
struct NewUser {
    name: Option<String>,
    email: String,
    password: String,
}

async fn add_user(pool: &MySqlPool, Query(params): Query<NewUser>) -> Json<UserBody<User>> {
    let user = query!(
        r#"
        INSERT INTO Users(name, email, password)
        VALUES (?, ?, ?);
        "#,
        params.name,
        params.email,
        params.password
    )
    .execute(pool)
    .await
    .unwrap();
    Json(UserBody {
        user: User {
            name: params.name,
            email: params.email,
            pasword: params.password,
        },
    })
}

async fn index() -> Html<&'static str> {
    println!("{:<12} - main_Site", "HANDLER");
    Html("<p>Pirate</p>")
}

async fn json() -> Json<Value> {
    println!("{:<12} - json", "HANDLER");
    Json(json!({
        "id": 0,
        "data": 42
    }))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("{:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}<strong"))
}

async fn hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("{:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}<strong"))
}
