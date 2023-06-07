pub mod error;

pub use error::Error;

pub trait App {
    fn load(&self) -> Result<()>;
}

pub type Result<T> = result_helper::ResultWrapper<T, Error>;

mod result_helper {
    pub type ResultWrapper<T, E> = Result<T, E>;
}