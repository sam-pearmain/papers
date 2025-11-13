use std::str::Chars;
use std::iter::Peekable;
use crate::bibtex::bibliography::Bibliography;
use crate::bibtex::entry::Entry;
use crate::bibtex::error::ParseError;
use crate::bibtex::fields::Field;

pub struct Parser<'a> {
    input: Peekable<Chars<'a>>, 
    row: u32, 
    col: u32, 
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { 
            input: input.chars().peekable(), 
            row: 1, col: 1,
        }
    }

    pub fn parse(&mut self) -> Result<Bibliography, ParseError> {
        let bibliography = Bibliography::new();
    }

    fn parse_entry(&mut self) -> Result<Entry, ParseError> {
        todo!()
    }

    fn parse_field(&mut self) -> Result<Field, ParseError> {
        todo!()
    }

    fn advance(&mut self) -> Option<char> {
        match self.input.next() {
            Some('\n') => {
                self.row += 1;
                self.col = 1;
                Some('\n')
            }
            Some(c) => {
                self.col += 1;
                Some(c)
            }
            None => None, 
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some('%') => {
                    self.advance();
                    while let Some(c) = self.advance() {
                        if c == '\n' {
                            break;
                        }
                    }
                }
                Some(c) if c.is_ascii_whitespace() => {
                    self.advance();
                }
                _ => break,
            }
        }
    }
}