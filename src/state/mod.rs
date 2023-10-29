pub mod cached;

use std::collections::HashMap;

use crate::{
    domain::{self, UserId},
    routes::models::{CreateUser, User},
};

/// State shared across all routes, our `cache`
pub struct AppState<T>(T)
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

// Here we are creating abstraction over CRUD like operations, because later on we can switch it
// with different cache implementation.
pub trait UsersCrud {
    fn create_user(id: UserId, data: domain::User);

    fn update_user(id: UserId, data: domain::UserPartial);

    fn delete_user(id: UserId, data: domain::UserPartial);

    fn get_user(id: UserId) -> domain::User;

    fn get_users() -> Vec<domain::User>;
}

// State non abstract:

// pub struct AppState {
//     users: HashMap<domain::UserId, domain::User>,
// }
