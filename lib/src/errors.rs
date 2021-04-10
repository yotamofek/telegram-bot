use std::error;
use std::fmt;

pub use telegram_bot_raw::Error as RawError;

#[derive(Debug)]
pub struct Error(ErrorKind);

#[derive(Debug)]
pub enum ErrorKind {
    Raw(RawError),
    Hyper(hyper::Error),
    Http(hyper::http::Error),
    Io(std::io::Error),
    InvalidMultipartFilename,
}

impl From<RawError> for ErrorKind {
    fn from(error: RawError) -> Self {
        ErrorKind::Raw(error)
    }
}

impl From<hyper::Error> for ErrorKind {
    fn from(error: hyper::Error) -> Self {
        ErrorKind::Hyper(error)
    }
}

impl From<hyper::http::Error> for ErrorKind {
    fn from(error: hyper::http::Error) -> Self {
        ErrorKind::Http(error)
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(error: std::io::Error) -> Self {
        ErrorKind::Io(error)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error(kind)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ErrorKind::Raw(error) => write!(f, "{}", error),
            ErrorKind::Hyper(error) => write!(f, "{}", error),
            ErrorKind::Http(error) => write!(f, "{}", error),
            ErrorKind::Io(error) => write!(f, "{}", error),
            ErrorKind::InvalidMultipartFilename => write!(f, "invalid multipart filename"),
        }
    }
}

impl error::Error for Error {}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}
