// lib.rs
use rocket::http::Status;
use rocket::State;
use rocket::response::status::{Custom, BadRequest};

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use sqlx::migrate::{Migrator};
use shuttle_service::{error::CustomError};

mod claims;

use claims::Claims;

#[macro_use]
extern crate rocket;

struct AppState {
    pool: PgPool,
}

#[derive(Serialize)]
struct PublicResponse {
    message: String,
}

#[derive(Serialize, FromRow)]
struct Note {
    pub note_id: i32,
    pub note: String,
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
            "Incorrect credentials.".to_string(),
        ));
    }

    let claim = Claims::from_name(&login.username);
    let response = LoginResponse {
        token: claim.into_token()?,
    };

    Ok(Json(response))
}

#[get("/notes")]
async fn get_notes_all(state: &State<AppState>) -> Result<Json<Vec<Note>>, BadRequest<String>> {
    let notes = sqlx::query_as("SELECT * FROM notes")
    .fetch_all(&state.pool)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(notes))
}

#[get("/notes/<note_id>")]
async fn get_notes(note_id: i32, state: &State<AppState>) -> Result<Json<Note>, BadRequest<String>> {
    let note = sqlx::query_as("SELECT * FROM notes WHERE note_id = $1")
    .bind(note_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(note))
}

static MIGRATOR: Migrator = sqlx::migrate!();

#[shuttle_service::main] 
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_service::ShuttleRocket {
    MIGRATOR.run(&pool).await.map_err(CustomError::new)?;

    let state = AppState {pool};
    let rocket = rocket::build().mount("/", routes![index, public, private, login, get_notes, get_notes_all]).manage(state);

    Ok(rocket)
}