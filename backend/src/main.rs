use axum::{
    extract::{Path, Query, State},
    middleware,
    response::{Html, Response},
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::Deserialize;
// use serde_json::{json, Value};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;
#[derive(Clone)]
struct AppState {
    pool: MySqlPool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let state = AppState { pool };
    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/api/user_a", post(add_user))
        .layer(middleware::map_response(main_response_mapper))
        .with_state(state);
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
    println!("{res:?}");
    res
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
    State(pool): State<AppState>,
    Json(params): Json<NewUser>,
) -> Json<UserBody<User>> {
    println!("{params:?}");
    let result = sqlx::query!(
        r#"
        INSERT INTO Users(name, email, password)
        VALUES (?, ?, ?);
        "#,
        params.name, params.email, params.password,
    )
    .execute(&pool.pool)
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
