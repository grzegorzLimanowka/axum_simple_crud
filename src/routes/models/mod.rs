/// These models are for `API`, they can (and in serious applications will) differ from domain models
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub name: String,
    pub surname: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUser {
    pub id: String,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub age: Option<u8>,
}
