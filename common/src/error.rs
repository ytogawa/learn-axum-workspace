use std::{backtrace::Backtrace, fmt::Display, string::FromUtf8Error};

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum Error {
    ExternalError(BoxError, Backtrace),
    NotFound(Option<BoxError>, Backtrace),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::ExternalError(Box::new(err), Backtrace::capture())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::ExternalError(Box::new(err), Backtrace::capture())
    }
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Error::ExternalError(Box::new(err), Backtrace::capture())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ExternalError(Box::new(err), Backtrace::capture())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ExternalError(_, _) => write!(f, "ExternalError"),
            Error::NotFound(_, _) => write!(f, "NotFound"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ExternalError(e, _) => Some(e.as_ref()),
            Error::NotFound(e, _) => Some(e.as_ref().unwrap().as_ref()),
        }
    }

    fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        match self {
            Error::ExternalError(_, b) => Some(b),
            Error::NotFound(_, b) => Some(b),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
