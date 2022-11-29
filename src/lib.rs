// lib.rs
use rocket::http::Status;
use rocket::State;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
// use sqlx::{Executor, FromRow, PgPool};

mod claims;

use claims::Claims;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
struct PublicResponse {
    message: String,
}

// #[derive(Serialize, FromRow)]
// struct Note {
//     pub id: i32,
//     pub user_id: i32,
//     pub note: String,
// }

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/public")]
fn public() -> Json<PublicResponse> {
    Json(PublicResponse {
        message: "This endpoint is open to anyone.".to_string(),
    })  
}

#[derive(Serialize)]
struct PrivateResponse {
    message: String,
    user: String,
}

#[get("/private")]
fn private(user: Claims) -> Json<PrivateResponse> {
    Json(PrivateResponse {
        message: "The `Claims` request guard ensures only valid JWTs can access this endpoint.".to_string(),
        user: user.name,
    })
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String
}

#[post("/login", data = "<login>")]
fn login(login: Json<LoginRequest>) -> Result<Json<LoginResponse>, Custom<String>> {
    if login.username != "username" || login.password != "password" {
        return Err(Custom(
            Status::Unauthorized,
            "Incorrect credentials.".to_string(),
        ));
    }

    let claim = Claims::from_name(&login.username);
    let response = LoginResponse {
        token: claim.into_token()?,
    };

    Ok(Json(response))
}

// #[get("/notes")]
// fn getNotes(pool: &State<PgPool>) -> Result<Json<Note>, E> {
//     let notes = sqlx::query("SELECT * FROM notes")
//     .execute(&PgPool)
//     .await
//     .map_err(|E| Status::InternalServerError)?;

//     Ok(Json(notes))
// }

#[shuttle_service::main] 
async fn rocket() -> shuttle_service::ShuttleRocket {
    // pool.execute(include_str!("../schema.sql"))
    //     .await
    //     .map_err(CustomError::new)?;

    // let state = MyState {pool};
    let rocket = rocket::build().mount("/", routes![index, public, private, login]);

    Ok(rocket)
}