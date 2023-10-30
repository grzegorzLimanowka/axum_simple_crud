use crate::{domain, routes};

impl TryFrom<domain::User> for routes::models::User {
    type Error = ();

    fn try_from(u: domain::User) -> Result<Self, Self::Error> {
        Ok(Self {
            id: u.id.0,
            name: u.name,
            surname: u.surname,
            age: u.age,
        })
    }
}

impl TryFrom<&mut domain::User> for routes::models::User {
    type Error = ();

    fn try_from(u: &mut domain::User) -> Result<Self, Self::Error> {
        Ok(Self {
            id: u.id.0.clone(),
            name: u.name.clone(),
            surname: u.surname.clone(),
            age: u.age,
        })
    }
}
