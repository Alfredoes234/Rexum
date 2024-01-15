use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use sqlx::FromRow;


#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}

#[derive(Serialize, Deserialize)]
pub struct UserBody<T> {
    pub user: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    pub id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
    pub verify: Option<bool>
}




#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub id: u64,
    pub name: Option<String>,
    pub email: String,
    pub password: String,
}