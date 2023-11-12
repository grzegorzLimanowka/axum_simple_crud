use std::collections::HashMap;

use axum::async_trait;

use crate::{
    domain::{self, User, UserId},
    error::AppError,
};

use super::UsersCrud;

/// State shared across all routes, our `cache`
pub struct CachedState {
    users: HashMap<domain::UserId, domain::User>,
}

impl CachedState {
    pub fn new() -> Self {
        Self {
            users: HashMap::<domain::UserId, domain::User>::new(),
        }
    }
}

// Implementation of `UsersCrud` trait for struct `CachedState`
#[async_trait]
impl UsersCrud for CachedState {
    async fn get_user(&self, id: UserId) -> Result<std::option::Option<domain::User>, AppError> {
        if let Some(user) = self.users.get(&id) {
            return Ok(Some(user.clone()));
        };

        Err(AppError::UserNotFound(id))
    }

    async fn get_users(&self) -> Result<std::option::Option<Vec<domain::User>>, AppError> {
        let users = self
            .users
            .iter()
            .map(|v| v.1.clone())
            .collect::<Vec<User>>();

        Ok(Some(users))
    }

    async fn create_user(&mut self, id: UserId, data: domain::User) -> Result<User, AppError> {
        if let Some(_) = self.users.get(&id) {
            return Err(AppError::UserAlreadyExist(id));
        }

        self.users.insert(id, data.clone());

        Ok(data)
    }

    async fn update_user(
        &mut self,
        id: UserId,
        data: domain::UserPartial,
    ) -> Result<User, AppError> {
        if let Some(user) = self.users.get_mut(&id) {
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
        if let Some(deleted) = self.users.remove(&id) {
            return Ok(deleted);
        }

        Err(AppError::UserNotFound(id))
    }
}
