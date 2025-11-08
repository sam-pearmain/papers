use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnknownEntryKind { kind: String },
    UnknownFieldKind { king: String }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnknownEntryKind { kind } => {
                write!(f, "unknown entry kind: {}", kind)
            }, 
            ParseError::UnknownFieldKind { king } => {
                write!(f, "unknown field kind: {}", kind)
            }
        }
    }
}

impl Error for ParseError {}