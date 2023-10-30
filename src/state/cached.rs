use std::collections::HashMap;

use crate::{
    domain::{self, UserId},
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

impl UsersCrud for CachedState {
    fn create_user(&mut self, id: UserId, data: domain::User) -> Result<(), AppError> {
        if let Some(user) = self.users.get(&id) {
            return Err(AppError::UserAlreadyExist(id));
        };

        self.users.insert(id, data);

        Ok(())
    }

    fn update_user(&mut self, id: UserId, data: domain::UserPartial) -> Result<(), AppError> {
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
        };

        Ok(())
    }

    fn delete_user(&mut self, id: UserId, data: domain::UserPartial) -> Result<(), AppError> {
        todo!()
    }

    fn get_user(&self, id: UserId) -> Result<std::option::Option<domain::User>, AppError> {
        todo!()
    }

    fn get_users(&self) -> Result<std::option::Option<Vec<domain::User>>, AppError> {
        todo!()
    }
}
