use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use crate::{
    error::AppError,
    models::user_models::{User, UserForCreate, UserForLogin, UserForUpdate},
    AppState,
};

#[derive(Debug)]
pub struct UserController {}

impl UserController {
    pub async fn create_user(
        State(app_state): State<Arc<AppState>>,
        Json(payload): Json<UserForCreate>,
    ) -> impl IntoResponse {
        let UserForCreate {
            username,
            email,
            password,
        } = payload;

        sqlx::query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)")
            .bind(username)
            .bind(email)
            .bind(password)
            .execute(&app_state.db_pool)
            .await
            .map_err(|_qe| AppError::DbError)
            .unwrap();

        "User created successfully"
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
