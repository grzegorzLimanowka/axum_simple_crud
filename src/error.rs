use thiserror::Error;

use crate::domain::UserId;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User with given id {0} already exists !")]
    UserAlreadyExist(UserId),
    #[error("User with given id {0} not found !")]
    UserNotFound(UserId),
    // TODO: Conversion error
}
