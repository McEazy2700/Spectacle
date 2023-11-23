use sea_orm::{DbConn, DbErr};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    io,
    sync::{MutexGuard, PoisonError},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppError {
    message: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.message);
    }
}

impl std::error::Error for AppError {
    fn description(&self) -> &str {
        return self.message.as_str();
    }
}

impl actix_web::error::ResponseError for AppError {}

impl AppError {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<actix_web::Error> for AppError {
    fn from(value: actix_web::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(value: DbErr) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<tauri::Error> for AppError {
    fn from(value: tauri::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<PoisonError<MutexGuard<'_, DbConn>>> for AppError {
    fn from(value: PoisonError<MutexGuard<'_, DbConn>>) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<regex::Error> for AppError {
    fn from(value: regex::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
