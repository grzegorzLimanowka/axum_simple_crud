//! All routes for `user` endpoint. they use shared `state` struct for state manipulations (CRUD ops)

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    domain::{self, UserId},
    state::{cached::CachedState, AppState, UsersCrud},
};

use super::models::{self, CreateUser, UpdateUser};

// TODO: Remove unwraps, return HTTP errors to client with proper codes

pub async fn get_users(State(state): State<Arc<Mutex<AppState<CachedState>>>>) -> Json<Value> {
    if let Ok(Some(users)) = state.lock().await.get_users().await {
        let u: Vec<models::User> = users
            .into_iter()
            .map(|u| models::User::try_from(u).unwrap())
            .collect();

        return Json(serde_json::to_value(u).unwrap());
    }

    Json(json!({}))
}

pub async fn get_user(
    State(state): State<Arc<Mutex<AppState<CachedState>>>>,
    Path(id): Path<String>,
) -> Json<Value> {
    let state = state.clone();
    let id = UserId::new(id);

    if let Ok(Some(user)) = state.lock().await.get_user(id).await {
        let u: models::User = user.try_into().unwrap();

        return Json(serde_json::to_value(u).unwrap());
    }

    Json(json!({}))
}

pub async fn create_user(
    State(state): State<Arc<Mutex<AppState<CachedState>>>>,
    Json(payload): Json<CreateUser>,
) {
    let mut state = state.clone();
    let id = UserId::new(uuid::Uuid::new_v4().to_string());

    // let user = domain::User {
    //     id,
    //     name: ,
    //     surname: todo!(),
    //     age: todo!(),
    // }

    // let user: domain::User = (id.clone(), payload).try_into().unwrap();

    // if let Ok(mutuser) = &mut state.create_user(id, user).await {
    //     //
    // }

    // if let Ok(Some(users)) = state.create_user(payload).await {
    //     let u: Vec<models::User> = users
    //         .into_iter()
    //         .map(|u| models::User::try_from(u).unwrap())
    //         .collect();

    //     return Json(serde_json::to_value(u).unwrap());
    // }

    // Json(json!({}))
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
