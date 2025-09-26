use std::{
    fmt, io,
    sync::{MutexGuard, PoisonError},
};

use crate::ioface::IoFace;

pub struct EString(String);

impl<T> From<T> for EString
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self(value.as_ref().to_string())
    }
}

#[derive(Debug)]
pub struct Error {
    details: String,
}

impl From<EString> for Error {
    fn from(err: EString) -> Self {
        Self { details: err.0 }
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self { details: err }
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self {
            details: err.to_string(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self {
            details: err.to_string(),
        }
    }
}

impl From<PoisonError<MutexGuard<'_, IoFace>>> for Error {
    fn from(err: PoisonError<MutexGuard<'_, IoFace>>) -> Self {
        Self {
            details: err.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Error: {}", self.details);
    }
}
