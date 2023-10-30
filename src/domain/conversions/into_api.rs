use crate::{domain, routes};

impl TryFrom<domain::User> for routes::models::User {
    type Error = ();

    fn try_from(value: domain::User) -> Result<Self, Self::Error> {
        todo!()
    }
}
