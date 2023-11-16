#[derive(Debug)]
pub enum NunjackError {
    NumParse(std::num::ParseIntError),
    StdIo(std::io::Error),
    Unknown(String),
}

impl PartialEq for NunjackError {
    fn eq(&self, other: &Self) -> bool {
        use NunjackError::*;

        match (self, other) {
            (NumParse(se), NumParse(oe)) => se == oe,
            (Unknown(ss), Unknown(os)) => ss == os,
            _ => false,
        }
    }
}

impl std::fmt::Display for NunjackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NumParse(err) => write!(f, "NumParse error: {err}"),
            Self::StdIo(err) => write!(f, "IO error: {err}"),
            Self::Unknown(msg) => write!(f, "Unknown nunjack error: {msg}"),
        }
    }
}

impl std::error::Error for NunjackError {}

impl From<std::io::Error> for NunjackError {
    fn from(value: std::io::Error) -> Self {
        NunjackError::StdIo(value)
    }
}

impl From<std::num::ParseIntError> for NunjackError {
    fn from(value: std::num::ParseIntError) -> Self {
        NunjackError::NumParse(value)
    }
}

pub type NunjackResult<T> = core::result::Result<T, NunjackError>;
