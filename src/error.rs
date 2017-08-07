use std::io;
use std::result;
use std::string;

pub enum Error {
    IO(io::Error),
    Utf8(string::FromUtf8Error),
    NotSupported(String)
}

pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(error: string::FromUtf8Error) -> Self {
        Error::Utf8(error)
    }
}
