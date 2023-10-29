use std::collections::HashMap;

use crate::domain::{self, UserId};

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
    fn create_user(id: UserId, data: domain::User) {
        todo!()
    }

    fn update_user(id: UserId, data: domain::UserPartial) {
        todo!()
    }

    fn delete_user(id: UserId, data: domain::UserPartial) {
        todo!()
    }

    fn get_user(id: UserId) -> domain::User {
        todo!()
    }

    fn get_users() -> Vec<domain::User> {
        todo!()
    }
}
