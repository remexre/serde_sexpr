use crate::Value;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A convenient alias for `Result`.
pub type Result<T> = std::result::Result<T, Error>;

/// An error while deserializing.
#[derive(Debug)]
pub enum Error {
    /// A custom error, from the serde `custom` methods.
    Custom(String),

    /// An IO error from `from_reader`/`to_writer`.
    Io(std::io::Error),

    /// An invalid s-expression was found when trying to deserialize the given Serde type.
    Invalid(&'static str, Value),

    /// A string failed to parse as an s-expression.
    ParseFailed,

    /// An s-expression was successfully parsed, but there was trailing input.
    ParseTrailing,

    /// An error converting bytes to UTF-8.
    Utf8(std::str::Utf8Error),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self {
            Error::Custom(s) => fmt.write_str(&s),
            Error::Io(err) => err.fmt(fmt),
            Error::Invalid(ty, val) => write!(fmt, "{} is not a {}", val, ty),
            Error::ParseFailed => fmt.write_str("parsing s-expression failed"),
            Error::ParseTrailing => fmt.write_str("parsing s-expression failed (trailing input)"),
            Error::Utf8(err) => err.fmt(fmt),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Utf8(err) => Some(err),
            _ => None,
        }
    }
}
