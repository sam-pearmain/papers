use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseErrorKind {
    UnknownEntryKind { kind: String },
    UnknownFieldKind { kind: String }, 
    BraceLevelExceeded,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownEntryKind { kind } => {
                write!(f, "unknown entry kind: {}", kind)
            }, 
            Self::UnknownFieldKind { kind } => {
                write!(f, "unknown field kind: {}", kind)
            }, 
            Self::BraceLevelExceeded => {
                write!(f, "brace level exceeded")
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind, 
    row: u32, 
    col: u32, 
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[row: {}, col {}] parse error \"{}\"", self.row, self.col, self.kind)
    }
}

impl Error for ParseError {}