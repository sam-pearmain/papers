use std::{fmt, error::Error};

#[derive(Debug)]
pub enum ParseFieldError {
    UnknownMonth { got: String }, 
}

impl fmt::Display for ParseFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownMonth { got } => {
                write!(f, "unknown month '{}'", got)
            }, 
        }
    }
}

impl Error for ParseFieldError {}