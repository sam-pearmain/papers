use std::error::Error;
use std::fmt;
use crate::bibtex::parser::Token;

#[derive(Debug)]
pub enum ParseErrorType {
    UnexpectedToken { token: Token}, 
    UnknownEntry { entry: String },
    UnknownField { field: String }, 
    BraceLevelExceeded,
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownEntry { entry } => {
                write!(f, "unknown entry: {}", entry)
            }, 
            Self::UnknownField { field } => {
                write!(f, "unknown field: {}", field)
            }, 
            Self::BraceLevelExceeded => {
                write!(f, "brace level exceeded")
            }, 
            Self::UnexpectedToken { token } => {
                write!(f, "unexpected token: {}", token)
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorType, 
    row: usize, 
    col: usize, 
}

impl ParseError {
    pub fn new(kind: ParseErrorType, row: usize, col: usize) -> Self {
        ParseError { kind, row, col }
    }

    pub fn unexpected_token(token: Token, row: usize, col: usize) -> Self {
        ParseError {
            kind: ParseErrorType::UnexpectedToken { token }, 
            row, col
        }
    }

    pub fn unknown_entry(entry: String, row: usize, col: usize) -> Self {
        ParseError { 
            kind: ParseErrorType::UnknownEntry { entry }, 
            row, col
        }
    }

    pub fn unknown_field(field: String, row: usize, col: usize) -> Self {
        ParseError {
            kind: ParseErrorType::UnknownField { field }, 
            row, col
        }
    }

    pub fn brace_level_exceeded(row: usize, col: usize) -> Self {
        ParseError {
            kind: ParseErrorType::BraceLevelExceeded, 
            row, col
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[row: {}, col {}] parse error \"{}\"", self.row, self.col, self.kind)
    }
}

impl Error for ParseError {}