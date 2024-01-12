use axum::{
    extract::State,
    middleware,
    response::{Html, Response},
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
// use serde_json::{json, Value};
use sqlx::mysql::MySqlPoolOptions;
use std::env;

mod structs;
use structs::*;

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
        .route("/api/usera", post(add_user))
        .route("/api/userd", post(delete_user))
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

async fn add_user(
    State(pool): State<AppState>,
    Json(params): Json<User>,
) -> Json<UserBody<User>> {
    println!("{params:?}");
    let result = sqlx::query!(
        r#"
        INSERT INTO Users(name, email, password)
        VALUES (?, ?, ?);
        "#,
        params.name,
        params.email,
        params.password,
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

async fn index() -> Html<&'static str> {
    println!("{:<12} - main_Site", "HANDLER");
    Html("<p>Pirate</p>")
}
