//! All routes for `user` endpoint. Simple CRUD operations
//!
//! adf
//! asdf
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    Json,
};

use crate::state::{cached::CachedState, AppState};

use super::models::{CreateUser, UpdateUser};

//state: Arc<AppState>

pub async fn get_users(State(state): State<Arc<Mutex<AppState<CachedState>>>>) {
    //
}

pub async fn get_user(
    State(state): State<Arc<Mutex<AppState<CachedState>>>>,
    Path(id): Path<String>,
) {
    //
}

pub async fn create_user(
    State(state): State<Arc<Mutex<AppState<CachedState>>>>,
    Json(payload): Json<CreateUser>,
) {
    //
}

pub async fn patch_user(
    State(state): State<Arc<Mutex<AppState<CachedState>>>>,
    Json(payload): Json<UpdateUser>,
) {
    //
}

pub async fn delete_user(State(state): State<Arc<Mutex<AppState<CachedState>>>>) {
    //
}
