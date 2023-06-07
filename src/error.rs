pub use anyhow as __anyhow;

use core::fmt;
use std::fmt::{Display, Debug};

/// A specialized `Result` type for doger.
pub struct Error {
    error: __anyhow::Error
}

/// Just like `anyhow::anyhow!(...)`, but automatically calls `doger::Error::from()` to spare some boilerplate.
#[macro_export]
macro_rules! anyhow {
    ($tt: tt) => {{<$crate::Error as From<$crate::error::__anyhow::Error>>::from($crate::error::__anyhow::anyhow!($tt))}};
}

impl Error {
    /// Create a new error object from any error type.
    ///
    /// The error type must be threadsafe and 'static, so that the Error will be
    /// as well. If the error type does not provide a backtrace, a backtrace will
    /// be created here to ensure that a backtrace exists.
    pub fn new<E>(error: E) -> Self
    where
        E: Into<__anyhow::Error>
    {
        Self { error: error.into() }
    }
    /// Create a new error object from static string.
    pub fn from_str<M>(msg: M) -> Self
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        Self {
            error: __anyhow::Error::msg(msg)
        }
    }
    pub fn into_inner(self) -> __anyhow::Error {
        self.error
    }
    /// Attempt to downcast the error object to a concrete type.
    pub fn downcast<E>(self) -> std::result::Result<E, Self>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if self.error.downcast_ref::<E>().is_some() {
            Ok(self.error.downcast().unwrap())
        } else {
            Err(self)
        }
    }
    /// Downcast this error object by reference.
    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        self.error.downcast_ref::<E>()
    }

    /// Downcast this error object by mutable reference.
    pub fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        self.error.downcast_mut::<E>()
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.error, formatter)
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.error, formatter)
    }
}

impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(error: E) -> Self {
        Self::new(error)
    }
}

impl AsRef<dyn std::error::Error + Send + Sync> for Error {
    fn as_ref(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        self.error.as_ref()
    }
}

impl AsRef<dyn std::error::Error> for Error {
    fn as_ref(&self) -> &(dyn std::error::Error + 'static) {
        self.error.as_ref()
    }
}

impl From<Error> for Box<dyn std::error::Error + Send + Sync + 'static> {
    fn from(error: Error) -> Self {
        error.error.into()
    }
}