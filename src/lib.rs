#![forbid(unsafe_code)]

pub mod error;
pub mod app;

pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::anyhow;
    pub use crate::app::{App, build as build_app};
}