use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct PrivateResponse {
    pub message: String,
    pub user: String,
}

#[derive(Serialize)]
pub struct PublicResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Note {
    pub note_id: i32,
    pub note: String,
    pub user_id: i32,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub imgsrc: String,
    pub gender: String,
    pub category: String,
    pub brand: String
}