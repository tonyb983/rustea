use std::sync::mpsc::{RecvError, SendError};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    SendRecv(String),
    Downcast(String),
}

impl Error {
    pub fn io(err: std::io::Error) -> Self {
        Error::Io(err)
    }

    pub fn send(err: String) -> Self {
        Error::SendRecv(err)
    }

    pub fn recv(err: String) -> Self {
        Error::SendRecv(err)
    }

    pub fn downcast<S: AsRef<str>>(err: S) -> Self {
        Error::Downcast(err.as_ref().to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::SendRecv(s) => write!(f, "Send/Recv error: {}", s),
            Error::Downcast(e) => write!(f, "Downcast error: {}", e),
        }
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(err: SendError<T>) -> Self {
        Self::SendRecv(err.to_string())
    }
}

impl From<RecvError> for Error {
    fn from(err: RecvError) -> Self {
        Self::SendRecv(err.to_string())
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
