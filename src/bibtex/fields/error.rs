use std::{fmt, error::Error};

#[derive(Debug)]
pub enum ParseFieldError {
    UnknownMonth { got: String }, 
    UnknownNumber { got: String }, 
    InvalidPageNumber { num: usize },
    InvalidYear { year: String },
    MangledPageRange { from: usize, to: usize },
}

impl fmt::Display for ParseFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownMonth { got } => {
                write!(f, "unknown month '{}'", got)
            }, 
            Self::UnknownNumber { got } => {
                write!(f, "cannot parse number '{}'", got)
            }, 
            Self::InvalidPageNumber { num } => {
                write!(f, "invalid page number '{}'", num)
            },
            Self::InvalidYear { year } => {
                write!(f, "invalid year '{}'", year)
            }, 
            Self::MangledPageRange { from, to } => {
                write!(f, "mangled page range '{}--{}'", from, to)
            }
        }
    }
}

impl Error for ParseFieldError {}