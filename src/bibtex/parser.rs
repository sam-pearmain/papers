use std::str::{Chars, FromStr};
use std::iter::Peekable;
use crate::bibtex::bibliography::Bibliography;
use crate::bibtex::entry::Entry;
use crate::bibtex::error::ParseError;

pub struct Parser<'a> {
    input: Peekable<Chars<'a>>, 
    row: usize, 
    col: usize, 
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { 
            input: input.chars().peekable(), 
            row: 1, col: 1,
        }
    }

    pub fn parse(&mut self) -> Result<Bibliography, ParseError> {
        let mut bibliography = Bibliography::new();

        loop {
            if let Some(entry) = self.parse_entry()? {
                bibliography.add(entry);
            } else {
                break;
            }
        }

        if bibliography.is_empty() {
            Err(ParseError::empty_input())
        } else {
            Ok(bibliography)
        }
    }

    fn parse_entry(&mut self) -> Result<Option<Entry>, ParseError> {
        todo!()
    }

    fn parse_fields(&mut self) {
        todo!()
    }

    fn parse_field(&mut self) -> Result<Field, ParseError> {
        let ident = self.consume_ident()?;
        let field = Field::from_str(&ident)
            .map_err(|_| ParseError::unknown_field(ident, self.row, self.col));
        
        self.consume_char('=')?;
        
        let value: String = self.consume_value()?;

        
    }

    /// Consumes an identifier
    fn consume_ident(&mut self) -> Result<String, ParseError> {
        self.skip_whitespace_and_comments();
        let mut ident = String::new();
        
        match self.peek() {
            Some(&c) => {
                if c.is_ascii_alphanumeric() {
                    self.advance();
                    ident.push(c);

                    loop {
                        if let Some(&c) = self.peek() {
                            self.advance();
                            ident.push(c);
                        } else {
                            break;
                        }
                    }
                } else {
                    // we expect an alphanumeric char to start the ident
                    return Err(ParseError::unexpected_char(c, self.row, self.col));
                }
            }, 
            None => {
                return Err(ParseError::unexpected_eof(self.row, self.col));
            }
        }

        Ok(ident)
    }

    /// Consumes a value contained within {}s or ""s
    fn consume_value(&mut self) -> Result<String, ParseError> {
        self.skip_whitespace_and_comments();

        // next char must be either a '{' or a '"'
        match self.peek() {
            Some(&c) => {
                match c {
                    '{' => self.consume_braced_value(),
                    '"' => self.consume_quoted_value(),
                    _   => Err(ParseError::unexpected_char(c, self.row, self.col))
                }
            }, 
            None => Err(ParseError::unexpected_eof(self.row, self.col))
        }
    }

    /// Consumes a "quoted" value and returns the raw string inside
    fn consume_quoted_value(&mut self) -> Result<String, ParseError> {
        self.consume_char('"')?;

        let mut value = String::new(); 

        loop {
            match self.advance() {
                Some('"') => {
                    break;
                }, 
                Some(c) => {
                    value.push(c);
                }, 
                None => {
                    return Err(ParseError::unexpected_eof(self.row, self.col))
                }
            }
        }

        if value.is_empty() {
            Err(ParseError::empty_value(self.row, self.col))
        } else {
            Ok(value)
        }
    }

    /// Consumes a {braced} value and returns the raw string inside
    fn consume_braced_value(&mut self) -> Result<String, ParseError> {
        self.consume_char('{')?;
        
        let mut value = String::new();
        let mut brace_level = 1;

        loop {
            match self.advance() {
                Some('\\') => {
                    // this is an escape character so push it and the next char
                    value.push('\\');
                    if let Some(c) = self.advance() {
                        value.push(c);
                    } else {
                        return Err(ParseError::unexpected_eof(self.row, self.col))
                    }
                }, 
                Some('{') => {
                    brace_level += 1;
                    value.push('{');
                }, 
                Some('}') => {
                    brace_level -= 1;
                    if brace_level == 0 {
                        // we have escaped the value
                        break;
                    } else {
                        value.push('}');
                    }
                }, 
                Some(c) => {
                    value.push(c);
                }, 
                None => {
                    return Err(ParseError::unexpected_eof(self.row, self.col));
                }
            }
        }

        if value.is_empty() {
            Err(ParseError::empty_value(self.row, self.col))
        } else {
            Ok(value)
        }
    }

    /// Consumes a given char
    fn consume_char(&mut self, ch: char) -> Result<(), ParseError> {
        self.skip_whitespace_and_comments();
        
        match self.peek() {
            Some(&c) => {
                if c == ch {
                    self.advance();
                    Ok(())
                } else {
                    Err(ParseError::unexpected_char(c, self.row, self.col))
                }
            }, 
            None => Err(ParseError::unexpected_eof(self.row, self.col))
        }
    }

    /// Advances the cursor forwards and consumes the current character
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

    /// Peeks the next char
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    /// Skips whitespace and inline comments
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