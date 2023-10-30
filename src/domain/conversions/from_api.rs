use crate::{
    domain::{self, UserId},
    routes::{
        self,
        models::{CreateUser, UpdateUser},
    },
};

impl TryFrom<routes::models::User> for domain::User {
    type Error = ();

    fn try_from(u: routes::models::User) -> Result<Self, Self::Error> {
        Ok(Self {
            id: UserId::new(u.id),
            name: u.name,
            surname: u.surname,
            age: u.age,
        })
    }
}

impl TryFrom<(UserId, CreateUser)> for domain::User {
    type Error = ();

    fn try_from(u: (UserId, CreateUser)) -> Result<Self, Self::Error> {
        Ok(Self {
            id: u.0,
            name: u.1.name,
            surname: u.1.surname,
            age: u.1.age,
        })
    }
}

impl TryFrom<(UserId, UpdateUser)> for domain::UserPartial {
    type Error = ();

    fn try_from(u: (UserId, UpdateUser)) -> Result<Self, Self::Error> {
        Ok(Self {
            id: u.0,
            name: u.1.name,
            surname: u.1.surname,
            age: u.1.age,
        })
    }
}
