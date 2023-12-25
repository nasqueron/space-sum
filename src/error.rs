use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::Error as IOError;

/// The error type for space-sum operations
pub enum SpaceSumError {
    /// The error variant for I/O operations
    IO(IOError),

    /// The error variant when parsing a value
    Parser(String),
}

impl From<IOError> for SpaceSumError {
    fn from(error: IOError) -> Self {
        Self::IO(error)
    }
}

impl Debug for SpaceSumError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(error) => std::fmt::Debug::fmt(&error, fmt),
            Self::Parser(error) => write!(fmt, "{error}"),
        }
    }
}

impl Display for SpaceSumError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(error) => std::fmt::Display::fmt(&error, fmt),
            Self::Parser(error) => write!(fmt, "{error}"),
        }
    }
}

impl Error for SpaceSumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IO(error) => error.source(),
            Self::Parser(_) => None,
        }
    }

    #[allow(deprecated)]
    fn description(&self) -> &str {
        match self {
            Self::IO(error) => error.description(),
            Self::Parser(error) => error,
        }
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            Self::IO(error) => error.cause(),
            Self::Parser(_) => None,
        }
    }
}
