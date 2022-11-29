// lib.rs
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

mod claims;

use claims::Claims;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
struct PublicResponse {
    message: String,
}

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
            "Account not found.".to_string(),
        ));
    }

    let claim = Claims::from_name(&login.username);
    let response = LoginResponse {
        token: claim.into_token()?,
    };

    Ok(Json(response))
}

#[shuttle_service::main] 
async fn rocket() -> shuttle_service::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, public, private, login]);

    Ok(rocket)
}