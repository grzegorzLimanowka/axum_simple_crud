#![allow(unused)]

use std::sync::{Arc, Mutex};

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    routes::users::{self, create_user, delete_user, get_user, get_users, patch_user},
    state::{cached::CachedState, AppState},
};

mod domain;
mod routes;
mod state;

const ADDR: &'static str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let cached_state = CachedState::new();

    let state = Arc::new(Mutex::new(AppState::<CachedState>::new(cached_state)));

    let app = Router::new()
        .route("/users", get(get_users))
        .with_state(Arc::clone(&state))
        .route("/users", post(create_user))
        .with_state(Arc::clone(&state))
        .route("/users/:id", get(get_user))
        .with_state(Arc::clone(&state))
        .route("/users/:id", patch(patch_user))
        .with_state(Arc::clone(&state))
        .route("/users/:id", delete(delete_user))
        .with_state(state);

    axum::Server::bind(&ADDR.parse().unwrap())
        .serve(app.into_make_service())
        .await;
}
