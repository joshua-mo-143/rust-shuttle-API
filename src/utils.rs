use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

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

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
