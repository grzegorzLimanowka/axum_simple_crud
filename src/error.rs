use thiserror::Error;

use crate::domain::UserId;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User with given id {0} already exists !")]
    UserAlreadyExist(UserId),
}
