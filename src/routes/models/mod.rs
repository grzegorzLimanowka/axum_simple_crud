/// These models are for `API`, they can differ from domain models
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    name: String,
    surname: String,
    age: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    name: String,
    surname: String,
    age: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUser {
    name: Option<String>,
    surname: Option<String>,
    age: Option<u8>,
}
