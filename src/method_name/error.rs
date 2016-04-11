use std::fmt;
use std::str;

#[derive(Debug)]
pub enum Error {
    InvalidLevel,
    NoSymbolName,
    NoMethodSeparator,
    Utf8(str::Utf8Error),
    Demangle(fmt::Error),
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Error {
        Error::Demangle(err)
    }
}
