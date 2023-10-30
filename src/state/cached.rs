use std::collections::HashMap;

use axum::async_trait;
use tokio::sync::Mutex;

use crate::{
    domain::{self, User, UserId},
    error::AppError,
};

use super::UsersCrud;

/// State shared across all routes, our `cache`
pub struct CachedState {
    users: Mutex<HashMap<domain::UserId, domain::User>>,
}

impl CachedState {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::<domain::UserId, domain::User>::new()),
        }
    }
}

// Implementation of `UsersCrud` trait for struct `CachedState`
#[async_trait]
impl UsersCrud for CachedState {
    async fn get_user(&self, id: UserId) -> Result<std::option::Option<domain::User>, AppError> {
        let users = self.users.lock().await;

        if let Some(user) = users.get(&id) {
            return Ok(Some(user.clone()));
        };

        Err(AppError::UserNotFound(id))
    }

    async fn get_users(&self) -> Result<std::option::Option<Vec<domain::User>>, AppError> {
        let users = self
            .users
            .lock()
            .await
            .iter()
            .map(|v| v.1.clone())
            .collect::<Vec<User>>();

        Ok(Some(users))
    }

    async fn create_user(&mut self, id: UserId, data: domain::User) -> Result<User, AppError> {
        let mut users = self.users.lock().await;

        if let Some(_) = users.get(&id) {
            return Err(AppError::UserAlreadyExist(id));
        }

        users.insert(id, data.clone());

        Ok(data)
    }

    async fn update_user(
        &mut self,
        id: UserId,
        data: domain::UserPartial,
    ) -> Result<User, AppError> {
        let mut users = self.users.lock().await;

        if let Some(user) = users.get_mut(&id) {
            if let Some(name) = data.name {
                user.name = name;
            }

            if let Some(surname) = data.surname {
                user.surname = surname;
            }

            if let Some(age) = data.age {
                user.age = age;
            }

            return Ok(user.clone());
        }

        Err(AppError::UserNotFound(id))
    }

    async fn delete_user(&mut self, id: UserId) -> Result<User, AppError> {
        let mut users = self.users.lock().await;

        if let Some(deleted) = users.remove(&id) {
            return Ok(deleted);
        }

        Err(AppError::UserNotFound(id))
    }
}
