use axum::{
    extract::State,
    middleware,
    response::{Html, Response},
    routing::{get, post},
    Json, Router,
};
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer, cookie::time::Duration};
use dotenv::dotenv;
use hashing::verify_password;
// use serde_json::{json, Value};
use sqlx::mysql::MySqlPoolOptions;
use std::env;

mod hashing;
use crate::hashing::hash_password;

mod structs;
use structs::*;

const AUTH_KEY: &str = "auth";

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Database stuff
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
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
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    // Ok
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
    println!("{params:?}");
    let hashed_pwd = hash_password(params.password);
    let hashed_password = hashed_pwd.map_err(|e| e.to_string()).unwrap();
    let result = sqlx::query!(
        r#"
        INSERT INTO Users(name, email, password)
        VALUES (?, ?, ?);
        "#,
        params.name,
        params.email,
        hashed_password,
    )
    .execute(&pool.pool)
    .await;

    match result {
        Ok(_) => Json(UserBody {
            user: User {
                name: params.name,
                email: params.email,
                password: hashed_password,
            },
        }),
        Err(e) => panic!("{e:?}"),
    }
}

async fn delete_user(State(pool): State<AppState>, Json(params): Json<Id>) -> Json<UserBody<Id>> {
    println!("{params:?}");
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
) -> Json<UserBody<Login>> {
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

    let body = result.as_ref().unwrap();
    let password = body.password.as_ref().unwrap().to_string();
    let email = body.email.as_ref().unwrap().to_string();

    let chec = verify_password(params.password, &password);

    match result {
        Ok(_) => {
            Json(UserBody {
                user: Login {
                    email: email,
                    password: password
                }
            })
        }
        Err(e) => panic!("{e:?}"),
    }
}

async fn index() -> Html<&'static str> {
    println!("{:<12} - main_Site", "HANDLER");
    Html("<p>Pirate</p>")
}
