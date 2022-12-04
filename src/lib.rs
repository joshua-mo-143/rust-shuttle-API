// lib.rs
use rocket::http::Status;
use rocket::State;
use rocket::response::status::{Custom, BadRequest};

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use sqlx::migrate::{Migrator};
use anyhow::Context as _;

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

#[derive(Deserialize, Serialize, FromRow)]
struct Note {
    pub note_id: i32,
    pub note: String,
    pub user_id: i32,
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

#[derive(Serialize, Deserialize)]
struct NewUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String
}

#[post("/register", data = "<new_user>")]
async fn register(new_user: Json<NewUser>, state: &State<AppState>) -> Result<(), BadRequest<String>> {
    let hashed_password = bcrypt::hash(&new_user.password, 7).unwrap();
    
    sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
                .bind(&new_user.username)
                .bind(hashed_password)
                .execute(&state.pool)
                .await
                .map_err(|e| BadRequest(Some(e.to_string())))?;
    
    Ok(())
                
}

#[post("/login", data = "<login>")]
async fn login(login: Json<LoginRequest>, state: &State<AppState>) -> Result<Json<LoginResponse>, Custom<String>> {
    let hashed_password = bcrypt::hash(&login.password, 7).unwrap();

    let _user_credentials = sqlx::query("SELECT username, password FROM users WHERE username = $1 AND password = $2")
    .bind(&login.username)
    .bind(hashed_password)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| Custom(Status::Unauthorized, e.to_string()));

    // if login.username != "username" || login.password != "password" {
    //     return Err(Custom(
    //         Status::Unauthorized,
    //         "Incorrect credentials.".to_string(),
    //     ));
    // }

    let claim = Claims::from_name(&login.username);
    let response = LoginResponse {
        token: claim.into_token()?,
    };

    Ok(Json(response))
}

#[get("/")]
async fn get_notes_all(state: &State<AppState>) -> Result<Json<Vec<Note>>, BadRequest<String>> {
    let notes = sqlx::query_as("SELECT * FROM notes")
    .fetch_all(&state.pool)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(notes))
}

#[get("/<note_id>")]
async fn get_notes_one(note_id: i32, state: &State<AppState>) -> Result<Json<Note>, BadRequest<String>> {
    let note = sqlx::query_as("SELECT * FROM notes WHERE note_id = $1")
    .bind(note_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(note))
}

#[get("/<user_id>/notes")]
async fn get_user_notes(user_id: i32, state: &State<AppState>) -> Result <Json<Vec<Note>>, BadRequest<String>> {
    let notes = sqlx::query_as("SELECT note_id, note, user_id FROM notes WHERE user_id = $1")
    .bind(user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(notes))
}

#[delete("/<note_id>")]
async fn delete_note(note_id: i32, state: &State<AppState>) -> Result<(), BadRequest<String>> {
    sqlx::query("DELETE FROM notes WHERE note_id = $1")
    .bind(note_id)
    .execute(&state.pool)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())));

    Ok(())
}

#[post("/", data = "<post_note>")]
async fn post_note(post_note: Json<Note>, state: &State<AppState>) -> Result<String, BadRequest<String>> {
    sqlx::query("INSERT INTO notes (note, user_id) VALUES ($1, $2)")
        .bind(&post_note.note)
        .bind(&post_note.user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())));

    Ok("added".to_string())
}

static MIGRATOR: Migrator = sqlx::migrate!();

#[shuttle_service::main] 
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_service::ShuttleRocket {
    MIGRATOR.run(&pool).await.context("Failed to run migrations")?;

    let state = AppState {pool};
    let rocket = rocket::build()
        .mount("/", routes![index, public, private, register, login])
        .mount("/users", routes![get_user_notes])
        .mount("/notes", routes![get_notes_all, get_notes_one, post_note, delete_note])
        .manage(state);

    Ok(rocket)
}