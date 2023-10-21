use anyhow::Error as AnyhowError;

/// A really basic error type
pub type Result<T> = std::result::Result<T, AnyhowError>;

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn default(err: impl Into<AnyhowError>) -> Self {
        Self {
            kind: ErrorKind::Default(err.into()),
        }
    }
    pub fn msg(msg: &str) -> Self {
        Self {
            kind: ErrorKind::Msg(msg.to_string()),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Default(err) => write!(f, "{:?}", err),
            ErrorKind::Msg(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Default(err) => write!(f, "{}", err),
            ErrorKind::Msg(msg) => write!(f, "{}", msg),
        }
    }
}

pub enum ErrorKind {
    Default(AnyhowError),
    Msg(String),
}
