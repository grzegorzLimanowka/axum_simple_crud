pub mod cached;

use std::collections::HashMap;

use axum::async_trait;

use crate::{
    domain::{self, User, UserId},
    error::AppError,
    routes::models::CreateUser,
};

/// State shared across all routes. Underlying type must implement `UsersCrud` trait
pub struct AppState<T>(T)
where
    T: UsersCrud + Send + Sync + 'static;

impl<T> AppState<T>
where
    T: UsersCrud + Send + Sync + 'static,
{
    pub fn new(state: T) -> Self {
        Self(state)
    }
}

// Here we are creating abstraction over CRUD like operations, because later on we can switch it
// with different cache implementation.
#[async_trait]
pub trait UsersCrud {
    async fn create_user(&mut self, id: UserId, data: domain::User) -> Result<(), AppError>;

    async fn update_user(&mut self, id: UserId, data: domain::UserPartial) -> Result<(), AppError>;

    async fn delete_user(
        &mut self,
        id: UserId,
        data: domain::UserPartial,
    ) -> Result<User, AppError>;

    async fn get_user(&self, id: UserId) -> Result<Option<domain::User>, AppError>;

    async fn get_users(&self) -> Result<Option<Vec<domain::User>>, AppError>;
}

// pub struct AppState {
//     users: HashMap<domain::UserId, domain::User>,
// }

/*

/// State shared across all routes. Underlying type must implement `UsersCrud` trait
pub struct AppState<T>(T)
where
    T: UsersCrud + Send + Sync + 'static;

impl<T> AppState<T>
where
    T: UsersCrud + Send + Sync + 'static,
{
    pub fn new(state: T) -> Self {
        Self(state)
    }
}

// Here we are creating abstraction over CRUD like operations, because later on we can switch it
// with different cache implementation.
#[async_trait]
pub trait UsersCrud<T>
where
    T: Send + Sync + 'static,
{
    async fn create_user(&mut self, id: UserId, data: domain::User) -> Result<(), AppError>;

    async fn update_user(&mut self, id: UserId, data: domain::UserPartial) -> Result<(), AppError>;

    async fn delete_user(
        &mut self,
        id: UserId,
        data: domain::UserPartial,
    ) -> Result<User, AppError>;

    async fn get_user(&self, id: UserId) -> Result<Option<domain::User>, AppError>;

    async fn get_users(&self) -> Result<Option<Vec<domain::User>>, AppError>;
}

// pub struct AppState {
//     users: HashMap<domain::UserId, domain::User>,
// }

*/
