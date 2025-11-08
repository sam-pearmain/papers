use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnknownEntryKind { kind: String }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnknownEntryKind { kind } => {
                write!(f, "unknown entry kind: {}", kind)
            }, 
        }
    }
}

impl Error for ParseError {}