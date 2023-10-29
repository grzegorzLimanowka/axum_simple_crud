pub struct UserId(String);

pub struct User {
    id: UserId,
    name: String,
    surname: String,
    age: u8,
}

pub struct UserPartial {
    id: UserId,
    name: Option<String>,
    surname: Option<String>,
    age: Option<u8>,
}
