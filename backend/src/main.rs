use anyhow::{Error, Result};
use axum::{
    debug_handler,
    extract::{Extension, Path, Query, FromRef, FromRequestParts},
    http::StatusCode,
    middleware,
    response::{Html,  Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
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

async fn pool() -> MySqlPool {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@localhost/main")
        .await
        .unwrap();
    return pool;
}

async fn main_response_mapper(res: Response) -> Response {
    println!("{:<12} - main_response_mapper", "RES_MAPPER");
    println!("{res:?}");
    res
}

fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/user_a", post(add_user))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    name: Option<String>,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct NewUser {
    name: Option<String>,
    email: String,
    password: String,
}

async fn add_user(
    Json(params): Json<NewUser>,
) -> Json<UserBody<User>> {
    let pool = pool().await;
    println!("{params:?}");
    let result = sqlx::query(
        "
        INSERT INTO Users(name, email, password)
        VALUES (?, ?, ?);
        ",
    )
    .bind(&params.name)
    .bind(&params.email)
    .bind(&params.password)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Json(UserBody {
            user: User {
                name: params.name,
                email: params.email,
                password: params.password,
            },
        }),
        Err(_) => panic!(),
    }
}

async fn index() -> Html<&'static str> {
    println!("{:<12} - main_Site", "HANDLER");
    Html("<p>Pirate</p>")
}
