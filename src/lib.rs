#![forbid(unsafe_code)]

pub mod error;
pub mod app;
pub mod request;

pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::anyhow;
    pub use crate::app::{App, AppBuilder, run as run_app};
    pub use crate::request::Request;
}