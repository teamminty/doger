#![forbid(unsafe_code)]

pub mod error;
pub mod app;

pub use error::Error;

pub type Result<T> = result_helper::ResultWrapper<T, Error>;

// This is needed, otherwise a recursive type is created.
mod result_helper {
    pub type ResultWrapper<T, E> = Result<T, E>;
}