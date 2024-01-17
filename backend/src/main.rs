use axum::{
    extract::State,
    middleware,
    response::{Html, Response},
    routing::{get, post},
    Json, Router, body::{Body, self}, http::StatusCode,
};
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer, cookie::time::Duration};
use dotenv::dotenv;
// use serde_json::{json, Value};
use sqlx::mysql::MySqlPoolOptions;
use std::env;

mod hashing;
use crate::hashing::{hash_password, verify_password};

mod structs;
use structs::*;
use crate::structs::{AppState, Login, UserBody};

// const AUTH_KEY: &str = "auth";

use anyhow::{Result, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Database stuff
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let state = AppState { pool };
    // Layers
    //let session_store = MemoryStore::default();
    //let session_layer = SessionManagerLayer::new(session_store)
    //    .with_secure(true)
    //    .with_expiry(Expiry::OnInactivity(Duration::seconds(2700)));
    // Build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/api/usera", post(add_user))
        .route("/api/userd", post(delete_user))
        .route("/api/login", post(login))
        .layer(middleware::map_response(main_response_mapper))
    //    .layer(session_layer)
        .with_state(state);
    // Run it
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await?;
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}


async fn main_response_mapper(res: Response) -> Response {
    println!("{:<12} - main_response_mapper", "RES_MAPPER");
    println!("{res:?}");
    res
}

async fn add_user(
    State(pool): State<AppState>,
    Json(params): Json<User>,
) -> Json<UserBody<User>> {
    // println!("{params:?}");
    let hashed_pwd = match hash_password(params.password) {
        Ok(password) => password,
        Err(err) => panic!("Error: {}", err.to_string()),
    };
    let result = sqlx::query!(
        r#"
        INSERT INTO Users(name, email, password)
        VALUES (?, ?, ?);
        "#,
        params.name,
        params.email,
        hashed_pwd,
    )
    .execute(&pool.pool)
    .await;

    match result {
        Ok(_) => Json(UserBody {
            user: User {
                name: params.name,
                email: params.email,
                password: hashed_pwd,
            },
        }),
        Err(e) => panic!("{e:?}"),
    }
}

async fn delete_user(State(pool): State<AppState>, Json(params): Json<Id>) -> Json<UserBody<Id>> {
    let result = sqlx::query!(
        r#"
        DELETE FROM Users
        WHERE id = (?)
        "#,
        params.id,
    )
    .execute(&pool.pool)
    .await;

    match result {
        Ok(_) => Json(UserBody {
            user: Id {
                id: params.id,
            },
        }),
        Err(e) => panic!("{e:?}"),
    }

}

async fn login(
    State(pool): State<AppState>,
    Json(params): Json<Login>
) -> Result<Json<UserBody<Login>>, Response<Body>> {
    let result = sqlx::query!(
        r#"
            SELECT email, password
            FROM Users
            WHERE email = ?;
            "#,
            params.email
        )
        .fetch_one(&pool.pool)
        .await;

    match result {
        Ok(_) => Ok(Json(UserBody {
            user: Login {
                email: params.email,
                password: params.password 
            },
        })),
        Err(_) => Err(Response::builder().status(StatusCode::UNAUTHORIZED).body(Body::empty()).unwrap())
    }   
}

async fn index() -> Html<&'static str> {
    println!("{:<12} - main_Site", "HANDLER");
    Html("<p>Pirate</p>")
}
