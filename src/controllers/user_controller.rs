use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    error::{AppError, Result},
    models::user_models::{User, UserForCreate, UserForLogin, UserForUpdate},
    responses::AppResponse,
    utils::hash_password,
    AppState,
};

#[derive(Debug)]
pub struct UserController {}

impl UserController {
    pub async fn create_user(
        State(app_state): State<Arc<AppState>>,
        Json(payload): Json<UserForCreate>,
    ) -> Result<AppResponse> {
        let UserForCreate {
            username,
            email,
            password,
        } = payload;

        let password = hash_password(&password)?;

        sqlx::query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)")
            .bind(username)
            .bind(email)
            .bind(password)
            .execute(&app_state.db_pool)
            .await
            .map_err(|qe| {
                if let sqlx::Error::Database(db_err) = &qe {
                    if db_err.constraint() == Some("users_email_key") {
                        AppError::DbError
                    } else if db_err.constraint() == Some("users_username_key") {
                        AppError::DbError
                    } else {
                        AppError::DbError
                    }
                } else {
                    AppError::DbError
                }
            })?;

        let body = json!({
            "message": "User created successfully"
        });

        Ok(AppResponse::new(StatusCode::CREATED, body))
    }

    pub async fn get_user_by_id(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<i32>,
    ) -> impl IntoResponse {
        let _user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&app_state.db_pool)
            .await
            .unwrap();

        "User found"
    }

    // pub async fn get_all_users(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    //     let _users = sqlx::query_as::<_, User>("SELECT * FROM users")
    //         .fetch_all(&app_state.db_pool)
    //         .await
    //         .unwrap();

    //     "Users found"
    // }

    pub async fn update_user(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<i32>,
        Json(payload): Json<UserForUpdate>,
    ) -> impl IntoResponse {
        let UserForUpdate {
            username,
            email,
            password,
        } = payload;

        let _user =
            sqlx::query("UPDATE users SET username = $1, email = $2, password = $3 WHERE id = $4")
                .bind(username)
                .bind(email)
                .bind(password)
                .bind(id)
                .execute(&app_state.db_pool)
                .await
                .map_err(|_qe| AppError::DbError)
                .unwrap();

        "User updated successfully"
    }

    pub async fn delete_user(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<i32>,
    ) -> impl IntoResponse {
        let _user = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&app_state.db_pool)
            .await
            .map_err(|_qe| AppError::DbError)
            .unwrap();

        "User deleted successfully"
    }

    pub async fn login_user(
        State(app_state): State<Arc<AppState>>,
        Json(payload): Json<UserForLogin>,
    ) -> impl IntoResponse {
        let UserForLogin { username, password } = payload;

        let _user =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1 AND password = $2")
                .bind(username)
                .bind(password)
                .fetch_one(&app_state.db_pool)
                .await
                .unwrap();

        "User logged in successfully"
    }
}
