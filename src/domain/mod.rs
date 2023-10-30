use std::{fmt::Display, ops::Deref};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl Deref for UserId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.0).as_str())
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub surname: String,
    pub age: u8,
}

#[derive(Hash, PartialEq, Eq)]
pub struct UserPartial {
    pub id: UserId,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub age: Option<u8>,
}
