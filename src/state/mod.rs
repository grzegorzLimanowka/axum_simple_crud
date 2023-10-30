pub mod cached;

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use axum::async_trait;

use crate::{
    domain::{self, User, UserId, UserPartial},
    error::AppError,
    routes::models::CreateUser,
};

// State shared across all routes. Underlying type must implement `UsersCrud` trait
pub struct AppState<T>(pub T)
where
    T: UsersCrud;

impl<T> AppState<T>
where
    T: UsersCrud,
{
    pub fn new(state: T) -> Self {
        Self(state)
    }
}

// Impl Deref for convinience, so we don't need to write `state.0`
impl<T> Deref for AppState<T>
where
    T: UsersCrud,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Required for mutations of wrapped structure
impl<T> DerefMut for AppState<T>
where
    T: UsersCrud,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Here we are creating abstraction over CRUD like operations, because later on we can switch it
// with different cache implementation.
#[async_trait]
pub trait UsersCrud {
    async fn get_user(&self, id: UserId) -> Result<Option<User>, AppError>;

    async fn get_users(&self) -> Result<Option<Vec<User>>, AppError>;

    async fn create_user(&mut self, id: UserId, data: User) -> Result<User, AppError>;

    async fn update_user(&mut self, id: UserId, data: UserPartial) -> Result<User, AppError>;

    async fn delete_user(&mut self, id: UserId) -> Result<User, AppError>;
}
